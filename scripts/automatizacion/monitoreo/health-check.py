#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
HEALTH-CHECK - Monitor de Salud
Verifica que el servidor responda y envía alertas si está caido.
Trigger: Cron cada 5 minutos
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import time
import requests
import json
from pathlib import Path
from datetime import datetime, timedelta

URLS = os.environ.get(
    "HEALTH_URLS", "http://localhost:3000/health,http://localhost:8000/health"
).split(",")
ALERT_THRESHOLD = int(os.environ.get("ALERT_THRESHOLD", "3"))
CHECK_INTERVAL = int(os.environ.get("CHECK_INTERVAL", "60"))
ALERT_FILE = Path("logs/health_alerts.json")


def load_alerts():
    if ALERT_FILE.exists():
        with open(ALERT_FILE, "r") as f:
            return json.load(f)
    return {}


def save_alerts(alerts):
    ALERT_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(ALERT_FILE, "w") as f:
        json.dump(alerts, f, indent=2)


def check_url(url):
    try:
        start = time.time()
        response = requests.get(url.strip(), timeout=10)
        elapsed = (time.time() - start) * 1000

        return {
            "status": "ok" if response.status_code < 500 else "error",
            "code": response.status_code,
            "latency_ms": int(elapsed),
        }
    except requests.exceptions.ConnectionError:
        return {"status": "down", "code": 0, "latency_ms": 0}
    except Exception as e:
        return {"status": "error", "code": 0, "latency_ms": 0, "error": str(e)}


def send_alert(url, failed_checks):
    msg = f"🚨 *ALERTA DE SALUD*\n\n*URL:* {url}\n*Checks fallidos:* {failed_checks}\n*Hora:* {datetime.now().strftime('%H:%M:%S')}"

    print(f"[ALERT] Enviando notificación para {url}")

    if "TELEGRAM_BOT_TOKEN" in os.environ and "TELEGRAM_CHAT_ID" in os.environ:
        try:
            requests.post(
                f"https://api.telegram.org/bot{os.environ['TELEGRAM_BOT_TOKEN']}/sendMessage",
                json={
                    "chat_id": os.environ["TELEGRAM_CHAT_ID"],
                    "text": msg,
                    "parse_mode": "Markdown",
                },
            )
        except:
            pass


def main():
    print("[HEALTH-CHECK] Monitor de Salud")
    print("=" * 50)

    alerts = load_alerts()
    issues = []

    for url in URLS:
        url = url.strip()
        if not url:
            continue

        print(f"[CHECK] {url}...")
        result = check_url(url)

        if result["status"] == "ok":
            print(
                f"   [OK] Status: {result['code']}, Latency: {result['latency_ms']}ms"
            )

            if url in alerts:
                print(f"   [OK] Recuperado! Eliminando alerta.")
                del alerts[url]
        else:
            print(f"   [ERROR] {result['status']}")

            if url not in alerts:
                alerts[url] = {"count": 1, "first_failure": datetime.now().isoformat()}
            else:
                alerts[url]["count"] += 1

            failed_count = alerts[url]["count"]

            if failed_count >= ALERT_THRESHOLD:
                if "last_alert" not in alerts[
                    url
                ] or datetime.now() - datetime.fromisoformat(
                    alerts[url]["last_alert"]
                ) > timedelta(hours=1):
                    send_alert(url, failed_count)
                    alerts[url]["last_alert"] = datetime.now().isoformat()

            issues.append(url)

    save_alerts(alerts)

    print()
    if issues:
        print(f"[WARNING] {len(issues)} endpoint(s) con problemas")
        return 1
    else:
        print("[OK] Todos los endpoints funcionando")
        return 0


if __name__ == "__main__":
    sys.exit(main())
