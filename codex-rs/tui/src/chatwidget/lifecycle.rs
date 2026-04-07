use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

use codex_app_server_protocol::AppSummary;
use codex_app_server_protocol::McpServerStartupState;
use codex_core::config::Constrained;
use codex_protocol::protocol::McpStartupStatus;
use codex_protocol::protocol::ThreadId;

use super::ChatWidget;
use super::history_cell;
use crate::app_command::AppCommand;
use crate::app_event::AppEvent;
use crate::app_server_session::ThreadSessionState;
use crate::app_server_session::thread_session_state_to_legacy_event;
use crate::bottom_pane::SelectionViewParams;
use crate::bottom_pane::standard_popup_hint_line;
use crate::chatwidget::TerminalTitleStatusKind;
use crate::history_cell::HistoryCell;
use crate::history_cell::PlainHistoryCell;
use crate::plan_type::PlanType;
use crate::status_account_display::StatusAccountDisplay;
use crate::thread_names::find_thread_name_by_id;

impl ChatWidget {
    pub(super) fn on_session_configured(
        &mut self,
        event: codex_protocol::protocol::SessionConfiguredEvent,
    ) {
        self.bottom_pane
            .set_history_metadata(event.history_log_id, event.history_entry_count);
        self.set_skills(/*skills*/ None);
        self.session_network_proxy = event.network_proxy.clone();
        self.thread_id = Some(event.session_id);
        self.thread_name = event.thread_name.clone();
        self.forked_from = event.forked_from_id;
        self.current_rollout_path = event.rollout_path.clone();
        self.current_cwd = Some(event.cwd.clone());
        match codex_core::types::AbsolutePathBuf::try_from(event.cwd.clone()) {
            Ok(cwd) => self.config.cwd = cwd,
            Err(err) => {
                tracing::warn!(path = %event.cwd.display(), %err, "session cwd should be absolute");
            }
        }
        if let Err(err) = self
            .config
            .permissions
            .approval_policy
            .set(event.approval_policy)
        {
            tracing::warn!(%err, "failed to sync approval_policy from SessionConfigured");
            self.config.permissions.approval_policy =
                Constrained::allow_only(event.approval_policy);
        }
        if let Err(err) = self
            .config
            .permissions
            .sandbox_policy
            .set(event.sandbox_policy.clone())
        {
            tracing::warn!(%err, "failed to sync sandbox_policy from SessionConfigured");
            self.config.permissions.sandbox_policy =
                Constrained::allow_only(event.sandbox_policy.clone());
        }
        self.config.approvals_reviewer = event.approvals_reviewer;
        self.status_line_project_root_name_cache = None;
        self.last_copyable_output = None;
        self.pending_turn_copyable_output = None;
        let forked_from_id = event.forked_from_id;
        let model_for_header = event.model.clone();
        self.session_header.set_model(&model_for_header);
        self.current_collaboration_mode = self.current_collaboration_mode.with_updates(
            Some(model_for_header.clone()),
            Some(event.reasoning_effort),
            /*developer_instructions*/ None,
        );
        if let Some(mask) = self.active_collaboration_mask.as_mut() {
            mask.model = Some(model_for_header.clone());
            mask.reasoning_effort = Some(event.reasoning_effort);
        }
        self.refresh_model_display();
        self.refresh_status_surfaces();
        self.sync_fast_command_enabled();
        self.sync_personality_command_enabled();
        self.sync_plugins_command_enabled();
        self.refresh_plugin_mentions();
        let startup_tooltip_override = self.startup_tooltip_override.take();
        let show_fast_status = self.should_show_fast_status(&model_for_header, event.service_tier);
        #[cfg(test)]
        let initial_messages = event.initial_messages.clone();
        let session_info_cell = history_cell::new_session_info(
            &self.config,
            &model_for_header,
            event,
            self.show_welcome_banner,
            startup_tooltip_override,
            self.plan_type,
            show_fast_status,
        );
        self.apply_session_info_cell(session_info_cell);

        #[cfg(test)]
        if let Some(messages) = initial_messages {
            self.replay_initial_messages(messages);
        }
        self.submit_op(AppCommand::list_skills(
            Vec::new(),
            /*force_reload*/ true,
        ));
        if self.connectors_enabled() {
            self.prefetch_connectors();
        }
        if let Some(user_message) = self.initial_user_message.take() {
            if self.suppress_initial_user_message_submit {
                self.initial_user_message = Some(user_message);
            } else {
                self.submit_user_message(user_message);
            }
        }
        if let Some(forked_from_id) = forked_from_id {
            self.emit_forked_thread_event(forked_from_id);
        }
        if !self.suppress_session_configured_redraw {
            self.request_redraw();
        }
    }

    pub(crate) fn set_initial_user_message_submit_suppressed(&mut self, suppressed: bool) {
        self.suppress_initial_user_message_submit = suppressed;
    }

    pub(crate) fn submit_initial_user_message_if_pending(&mut self) {
        if let Some(user_message) = self.initial_user_message.take() {
            self.submit_user_message(user_message);
        }
    }

    pub(crate) fn handle_thread_session(&mut self, session: ThreadSessionState) {
        self.on_session_configured(thread_session_state_to_legacy_event(session));
    }

    pub(super) fn emit_forked_thread_event(&self, forked_from_id: ThreadId) {
        let app_event_tx = self.app_event_tx.clone();
        let codex_home = self.config.codex_home.clone();
        tokio::spawn(async move {
            let forked_from_id_text = forked_from_id.to_string();
            let send_name_and_id = |name: String| {
                let line: Line<'static> = vec![
                    "• ".dim(),
                    "Thread forked from ".into(),
                    name.cyan(),
                    " (".into(),
                    forked_from_id_text.clone().cyan(),
                    ")".into(),
                ]
                .into();
                app_event_tx.send(AppEvent::InsertHistoryCell(Box::new(
                    PlainHistoryCell::new(vec![line]),
                )));
            };
            let send_id_only = || {
                let line: Line<'static> = vec![
                    "• ".dim(),
                    "Thread forked from ".into(),
                    forked_from_id_text.clone().cyan(),
                ]
                .into();
                app_event_tx.send(AppEvent::InsertHistoryCell(Box::new(
                    PlainHistoryCell::new(vec![line]),
                )));
            };

            match find_thread_name_by_id(&codex_home, &forked_from_id).await {
                Ok(Some(name)) if !name.trim().is_empty() => {
                    send_name_and_id(name);
                }
                Ok(_) => send_id_only(),
                Err(err) => {
                    tracing::warn!("Failed to read forked thread name: {err}");
                    send_id_only();
                }
            }
        });
    }

    pub(super) fn on_thread_name_updated(
        &mut self,
        event: codex_protocol::protocol::ThreadNameUpdatedEvent,
    ) {
        if self.thread_id == Some(event.thread_id) {
            self.thread_name = event.thread_name;
            self.refresh_terminal_title();
            self.request_redraw();
        }
    }

    pub(super) fn set_skills(&mut self, skills: Option<Vec<codex_protocol::protocol::SkillMetadata>>) {
        self.bottom_pane.set_skills(skills);
    }

    pub(super) fn update_mcp_startup_status(
        &mut self,
        update: codex_app_server_protocol::McpStartupUpdate,
    ) {
        if self.mcp_startup_ignore_updates_until_next_start {
            // After initial startup settles, core might emit stale terminal updates for already
            // running servers (or those shutting down). We only re-enter startup tracking once a
            // notification carries `Starting` for an expected server (or once we've seen enough
            // terminal updates to confirm a whole new round).
            if update.state == McpServerStartupState::Starting
                && self
                    .mcp_startup_expected_servers
                    .as_ref()
                    .map(|expected| expected.contains(&update.server_name))
                    .unwrap_or(false)
            {
                tracing::info!(server = %update.server_name, "exiting mcp_startup ignore window on Starting update");
                self.mcp_startup_ignore_updates_until_next_start = false;
                self.on_mcp_startup_complete();
            } else {
                // Buffer terminal/ready updates that occur between rounds. If we collect status
                // for all expected servers and any of them was `Starting`, we restart tracking.
                if let Some(expected) = &self.mcp_startup_expected_servers {
                    if expected.contains(&update.server_name) {
                        if update.state == McpServerStartupState::Starting {
                            self.mcp_startup_pending_next_round_saw_starting = true;
                        }
                        self.mcp_startup_pending_next_round
                            .insert(update.server_name.clone(), McpStartupStatus::from(update.state));
                        if self.mcp_startup_pending_next_round.len() >= expected.len() {
                            if self.mcp_startup_pending_next_round_saw_starting
                                || self.mcp_startup_allow_terminal_only_next_round
                            {
                                tracing::info!("restarting mcp_startup tracking on fresh round update");
                                let pending = std::mem::take(&mut self.mcp_startup_pending_next_round);
                                self.mcp_startup_status = Some(pending);
                                self.mcp_startup_ignore_updates_until_next_start = false;
                                self.mcp_startup_pending_next_round_saw_starting = false;
                                self.mcp_startup_allow_terminal_only_next_round = false;
                                self.on_mcp_startup_update();
                            } else {
                                // Full set of ready/terminal updates arrived but none were starting;
                                // keep ignoring but clear buffer for next sweep.
                                self.mcp_startup_pending_next_round.clear();
                            }
                        }
                    }
                }
                return;
            }
        }

        let status = self.mcp_startup_status.get_or_insert_with(HashMap::new);
        status.insert(update.server_name, McpStartupStatus::from(update.state));

        // If all expected servers have reached a terminal or ready state, we're done.
        if let Some(expected) = &self.mcp_startup_expected_servers {
            let all_ready_or_terminal = expected.iter().all(|name| {
                status
                    .get(name)
                    .map(|s| s.is_ready() || s.is_terminal())
                    .unwrap_or(false)
            });

            if all_ready_or_terminal {
                // If every server successfully reached 'Ready', we can finalize immediately.
                // Otherwise (some failed/stuck), we wait a small lag to see if any late updates
                // arrive before settling on the failure state.
                let all_ready = expected
                    .iter()
                    .all(|name| status.get(name).map(|s| s.is_ready()).unwrap_or(false));
                if all_ready {
                    self.finish_mcp_startup();
                } else {
                    self.finish_mcp_startup_after_lag();
                }
            }
        }

        self.on_mcp_startup_update();
    }

    pub(super) fn set_mcp_startup_expected_servers(&mut self, servers: HashSet<String>) {
        if !servers.is_empty() {
            self.mcp_startup_expected_servers = Some(servers);
        }
    }

    pub(super) fn on_mcp_startup_update(&mut self) {
        self.update_task_running_header();
        self.request_redraw();
    }

    pub(super) fn finish_mcp_startup(&mut self) {
        if self.mcp_startup_status.is_some() {
            tracing::info!("mcp_startup settled");
            self.mcp_startup_status = None;
            self.mcp_startup_ignore_updates_until_next_start = true;
            self.mcp_startup_pending_next_round.clear();
            self.mcp_startup_pending_next_round_saw_starting = false;
            // Once settled, we accept terminal-only rounds until the next explicit Start happens.
            self.mcp_startup_allow_terminal_only_next_round = true;
            self.on_mcp_startup_complete();
        }
    }

    pub(super) fn finish_mcp_startup_after_lag(&mut self) {
        let app_event_tx = self.app_event_tx.clone();
        // 350ms of lag is enough to let any "retrying" or late-arriving exit codes settle before we
        // drop the startup overlay.
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(350)).await;
            app_event_tx.send(AppEvent::McpStartupSettled);
        });
    }

    pub(super) fn on_mcp_startup_complete(&mut self) {
        self.update_task_running_header();
        self.request_redraw();
    }

    pub(super) fn on_mcp_server_status_updated(
        &mut self,
        event: McpServerStatusUpdatedNotification,
    ) {
        // If we are currently in a startup tracking round, let that logic handle it.
        // It provides better coalesced state for the status header.
        if self.mcp_startup_status.is_some() {
            return;
        }

        // Otherwise, show a transient header for the status change.
        match event.state {
            McpServerStartupState::Ready => {
                self.terminal_title_status_kind = TerminalTitleStatusKind::Thinking;
                self.set_status_header(format!("{} ready", event.server_name));
            }
            McpServerStartupState::Failed | McpServerStartupState::Exited => {
                self.terminal_title_status_kind = TerminalTitleStatusKind::Error;
                self.set_status_header(format!("{} failed", event.server_name));
            }
            _ => {}
        }
        self.request_redraw();
    }

    fn update_task_running_header(&mut self) {
        let is_running = self.agent_turn_running || self.mcp_startup_status.is_some();
        if is_running {
            self.terminal_title_status_kind = TerminalTitleStatusKind::Thinking;
            if let Some(status) = &self.mcp_startup_status {
                // If MCP is starting, show those details.
                let mut servers: Vec<_> = status.keys().collect();
                servers.sort();
                let summary = servers
                    .iter()
                    .map(|name| {
                        let ok = status.get(*name).map(|s| s.is_ready()).unwrap_or(false);
                        if ok {
                            format!("{name} ✓")
                        } else {
                            name.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                self.set_status_header(format!("Starting MCP: {summary}"));
            } else {
                // Ordinary turn header.
                self.set_status_header("Thinking...".to_string());
            }
        } else {
            self.terminal_title_status_kind = TerminalTitleStatusKind::Idle;
            self.set_status_header("Ready".to_string());
        }
    }
}
