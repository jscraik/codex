import json
import os

rust_files = []
target_ignore = ["target", ".git", "node_modules", "dist"]

for root, dirs, files in os.walk("codex-rs"):
    dirs[:] = [d for d in dirs if d not in target_ignore]
    for file in files:
        if file.endswith(".rs"):
            path = os.path.join(root, file)
            try:
                with open(path, 'r', errors='ignore') as f:
                    lines = sum(1 for _ in f)
                    rust_files.append({
                        "name": path,
                        "stats": {"code": lines}
                    })
            except:
                pass

output = {
    "Rust": {
        "reports": rust_files
    }
}

os.makedirs(".recon/runs/current/raw/loc", exist_ok=True)
with open(".recon/runs/current/raw/loc/tokei.json", "w") as out:
    json.dump(output, out, indent=2)

print("LOC counted manually via Python.")
