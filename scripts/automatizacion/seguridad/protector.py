#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
PROTECTOR - Seguridad Perimetral (IP Baner)
Detecta ataques de fuerza bruta y bloquea IPs maliciosas.
Trigger: Cron cada 5 minutos o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import re
import subprocess
from collections import defaultdict
from pathlib import Path
from datetime import datetime, timedelta

LOG_FILE = os.environ.get("AUTH_LOG", "/var/log/auth.log")
MAX_ATTEMPTS = int(os.environ.get("MAX_ATTEMPTS", "5"))
BAN_DURATION_HOURS = int(os.environ.get("BAN_DURATION_HOURS", "24"))
BANS_FILE = Path("logs/ip_bans.json")


def load_bans():
    import json

    if BANS_FILE.exists():
        with open(BANS_FILE, "r") as f:
            return json.load(f)
    return {}


def save_bans(bans):
    import json

    BANS_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(BANS_FILE, "w") as f:
        json.dump(bans, f, indent=2)


def get_failed_attempts():
    attempts = defaultdict(int)

    if not Path(LOG_FILE).exists():
        print(f"[WARNING] Log file not found: {LOG_FILE}")
        return attempts

    try:
        with open(LOG_FILE, "r", encoding="utf-8", errors="ignore") as f:
            lines = f.readlines()

        for line in lines[-1000:]:
            if "Failed password" in line or "authentication failure" in line:
                ip_match = re.search(r"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})", line)
                if ip_match:
                    ip = ip_match.group(1)
                    if not ip.startswith("127."):
                        attempts[ip] += 1
    except PermissionError:
        print(f"[WARNING] Permission denied reading {LOG_FILE}")
        print("[INFO] Try running with sudo or set AUTH_LOG to a readable path")

    return attempts


def ban_ip(ip):
    import json

    bans = load_bans()
    bans[ip] = {
        "banned_at": datetime.now().isoformat(),
        "duration_hours": BAN_DURATION_HOURS,
        "reason": "Brute force attack",
    }
    save_bans(bans)

    try:
        subprocess.run(
            ["iptables", "-A", "INPUT", "-s", ip, "-j", "DROP"],
            capture_output=True,
            check=True,
        )
        print(f"   [OK] IP {ip} blocked with iptables")
    except:
        print(f"   [INFO] Could not ban with iptables (need sudo)")
        print(f"   [INFO] Add to /etc/hosts.deny: ALL: {ip}")


def unban_expired():
    import json

    bans = load_bans()
    now = datetime.now()
    expired = []

    for ip, data in bans.items():
        banned_at = datetime.fromisoformat(data["banned_at"])
        duration = timedelta(hours=data["duration_hours"])

        if now - banned_at > duration:
            expired.append(ip)

    for ip in expired:
        del bans[ip]
        try:
            subprocess.run(
                ["iptables", "-D", "INPUT", "-s", ip, "-j", "DROP"], capture_output=True
            )
            print(f"   [OK] IP {ip} unbanned")
        except:
            pass

    if expired:
        save_bans(bans)


def main():
    print("[PROTECTOR] Seguridad Perimetral")
    print("=" * 50)
    print(f"[INFO] Max attempts: {MAX_ATTEMPTS}")
    print(f"[INFO] Log file: {LOG_FILE}")
    print()

    unban_expired()

    print("[INFO] Scanning for failed login attempts...")
    attempts = get_failed_attempts()

    if not attempts:
        print("[OK] No suspicious activity detected")
        return 0

    print(f"[ALERT] Found {len(attacks)} IPs with failed attempts:")

    ips_to_ban = []
    for ip, count in sorted(attempts.items(), key=lambda x: x[1], reverse=True):
        if count >= MAX_ATTEMPTS:
            ips_to_ban.append(ip)
            print(f"   [BAN] {ip}: {count} failed attempts")
        else:
            print(f"   [WARN] {ip}: {count} failed attempts")

    print()

    for ip in ips_to_ban:
        ban_ip(ip)

    if ips_to_ban:
        print(f"\n[OK] {len(ips_to_ban)} IPs banned")
    else:
        print("[OK] No IPs need to be banned")

    return 0


if __name__ == "__main__":
    attacks = get_failed_attempts()
    sys.exit(main())
