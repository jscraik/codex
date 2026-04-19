#!/usr/bin/env python3
import json
import os
import re
import sys

def main():
    # Load normalized metrics
    metrics_path = '.recon/runs/current/derived/metrics.json'
    if not os.path.exists(metrics_path):
        print(f"Error: {metrics_path} not found. Run recon collection first.")
        sys.exit(1)
        
    with open(metrics_path) as f:
        metrics_list = json.load(f)
        
    metrics_db = {}
    for m in metrics_list:
        metrics_db[(m['metric'], m['scope'])] = str(m['value'])

    # Load LOC data
    tokei_path = '.recon/runs/current/raw/loc/tokei.json'
    tokei_stats = {}
    if os.path.exists(tokei_path):
        with open(tokei_path) as f:
            tokei = json.load(f)
            reports = tokei.get("Rust", {}).get("reports", [])
            for r in reports:
                # normalize path somewhat
                path = r['name']
                tokei_stats[path] = str(r['stats']['code'])

    docs_dir = 'docs'
    if not os.path.exists(docs_dir):
        print("No docs/ directory found.")
        sys.exit(0)
        
    has_errors = False
    
    # E.g. 860 <!-- check-metric: wildcard_match_arms_naive | codex-rs (repo-wide) -->
    metric_regex = re.compile(r'([\d,]+)\s*<!--\s*check-metric:\s*(.+?)\s*\|\s*(.+?)\s*-->')
    
    # E.g. 11,071 LOC <!-- check-loc: codex-rs/tui/src/chatwidget.rs -->
    loc_regex = re.compile(r'([\d,]+)\s+LOC.*<!--\s*check-loc:\s*(.+?)\s*-->')

    print("Running markdown contradiction checker...")
    for filename in os.listdir(docs_dir):
        if not filename.endswith('.md'):
            continue
            
        filepath = os.path.join(docs_dir, filename)
        with open(filepath, 'r') as f:
            lines = f.readlines()
            
        for line_idx, line in enumerate(lines):
            # Evaluate explicit metric claims
            for match in metric_regex.finditer(line):
                val_str, metric_name, scope = match.groups()
                claimed_val = val_str.replace(',', '')
                metric_name = metric_name.strip()
                scope = scope.strip()
                
                key = (metric_name, scope)
                if key not in metrics_db:
                    print(f"[{filepath}:{line_idx+1}]   ❌ Error: Unknown metric or scope queried: {key}")
                    has_errors = True
                    continue
                
                actual_val = metrics_db[key]
                if claimed_val != actual_val:
                    print(f"[{filepath}:{line_idx+1}]   ❌ Contradiction! Markdown claims {val_str} for '{metric_name}', but canonical source says {actual_val}.")
                    has_errors = True

            # Evaluate exact LOC claims
            for match in loc_regex.finditer(line):
                val_str, path = match.groups()
                claimed_loc = val_str.replace(',', '')
                path = path.strip()
                
                # Suffix match for path since tokei paths might be long
                matched_actual = None
                for actual_path, loc in tokei_stats.items():
                    if actual_path.endswith(path):
                        matched_actual = loc
                        break
                        
                if matched_actual is None:
                    print(f"[{filepath}:{line_idx+1}]   ❌ Error: LOC check failed -> path '{path}' not found in tokei output.")
                    has_errors = True
                    continue
                    
                actual_loc = matched_actual
                if claimed_loc != actual_loc:
                    print(f"[{filepath}:{line_idx+1}]   ❌ Contradiction! Markdown claims {val_str} LOC for '{path}', but canonical source says {actual_loc}.")
                    has_errors = True

    if has_errors:
        print("\nFAIL: Documents contradict reality. Please update the numbers or run recon gathering.")
        sys.exit(1)
    else:
        print("\nSUCCESS: All document claims align perfectly with canonical metrics.")
        sys.exit(0)

if __name__ == '__main__':
    main()
