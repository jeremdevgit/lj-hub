#!/usr/bin/env python3
"""Build a factual device-ID catalog from a checked-out libratbag tree.

Usage: python3 tools/refresh_libratbag_catalog.py /path/to/libratbag
This records upstream identifiers, not a claim that LJ Hub can control a device.
"""

import json
import pathlib
import re
import subprocess
import sys


def main() -> None:
    root = pathlib.Path(sys.argv[1]).resolve()
    files = root / "data" / "devices"
    if not files.is_dir():
        raise SystemExit("Expected libratbag/data/devices")
    commit = subprocess.check_output(["git", "-C", str(root), "rev-parse", "HEAD"], text=True).strip()
    rows = []
    for path in sorted(files.glob("*.device")):
        props = {}
        for line in path.read_text(encoding="utf-8").splitlines():
            if line.startswith("[") and line != "[Device]":
                break
            if "=" in line and not line.lstrip().startswith("#"):
                key, value = line.split("=", 1)
                props[key.strip()] = value.strip()
        for match in props.get("DeviceMatch", "").split(";"):
            found = re.fullmatch(r"(usb|bluetooth):046d:([0-9a-fA-F]{4})", match)
            if found:
                rows.append({
                    "name": props.get("Name", "Logitech device"),
                    "transport": found.group(1),
                    "product_id": found.group(2).upper(),
                    "kind": props.get("DeviceType", "other"),
                    "driver": props.get("Driver", "unknown"),
                })
    output = {"source": "https://github.com/libratbag/libratbag", "commit": commit, "devices": rows}
    target = pathlib.Path(__file__).resolve().parents[1] / "data" / "libratbag-logitech.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"Wrote {len(rows)} identifier entries to {target}")


if __name__ == "__main__":
    main()
