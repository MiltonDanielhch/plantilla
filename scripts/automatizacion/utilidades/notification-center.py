#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
NOTIFICATION-CENTER - Centro de Notificaciones
Envía notificaciones a múltiples canales: Telegram, Email, Slack, Discord.
Trigger: Cualquier script que necesite alertar
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import requests
from pathlib import Path
from datetime import datetime

CHANNELS = {
    "telegram": lambda msg, config: send_telegram(msg, config),
    "email": lambda msg, config: send_email(msg, config),
    "slack": lambda msg, config: send_slack(msg, config),
    "discord": lambda msg, config: send_discord(msg, config),
}


def send_telegram(message, config):
    if not config.get("token") or not config.get("chat_id"):
        return False, "Telegram no configurado"

    try:
        response = requests.post(
            f"https://api.telegram.org/bot{config['token']}/sendMessage",
            json={
                "chat_id": config["chat_id"],
                "text": message,
                "parse_mode": "Markdown",
            },
            timeout=10,
        )
        return response.status_code == 200, "Enviado"
    except Exception as e:
        return False, str(e)


def send_email(message, config):
    if not config.get("smtp_host"):
        return False, "Email no configurado"

    try:
        import smtplib
        from email.mime.text import MIMEText

        msg = MIMEText(message, "plain", "utf-8")
        msg["Subject"] = f"[Sintonía 3026] {datetime.now().strftime('%H:%M')}"
        msg["From"] = config.get("from", "sintonia@local")
        msg["To"] = config.get("to", "admin@local")

        server = smtplib.SMTP(config["smtp_host"], config.get("smtp_port", 587))
        server.starttls()

        if config.get("username"):
            server.login(config["username"], config["password"])

        server.send_message(msg)
        server.quit()

        return True, "Enviado"
    except Exception as e:
        return False, str(e)


def send_slack(message, config):
    if not config.get("webhook"):
        return False, "Slack no configurado"

    try:
        response = requests.post(config["webhook"], json={"text": message}, timeout=10)
        return response.status_code == 200, "Enviado"
    except Exception as e:
        return False, str(e)


def send_discord(message, config):
    if not config.get("webhook"):
        return False, "Discord no configurado"

    try:
        response = requests.post(
            config["webhook"], json={"content": message}, timeout=10
        )
        return response.status_code in [200, 204], "Enviado"
    except Exception as e:
        return False, str(e)


def load_config():
    config = {}

    if os.environ.get("TELEGRAM_BOT_TOKEN"):
        config["telegram"] = {
            "token": os.environ["TELEGRAM_BOT_TOKEN"],
            "chat_id": os.environ.get("TELEGRAM_CHAT_ID"),
        }

    if os.environ.get("SMTP_HOST"):
        config["email"] = {
            "smtp_host": os.environ["SMTP_HOST"],
            "smtp_port": int(os.environ.get("SMTP_PORT", "587")),
            "username": os.environ.get("SMTP_USERNAME"),
            "password": os.environ.get("SMTP_PASSWORD"),
            "from": os.environ.get("EMAIL_FROM"),
            "to": os.environ.get("EMAIL_TO"),
        }

    if os.environ.get("SLACK_WEBHOOK"):
        config["slack"] = {"webhook": os.environ["SLACK_WEBHOOK"]}

    if os.environ.get("DISCORD_WEBHOOK"):
        config["discord"] = {"webhook": os.environ["DISCORD_WEBHOOK"]}

    return config


def main():
    import argparse

    parser = argparse.ArgumentParser(description="Notification Center")
    parser.add_argument("-m", "--message", required=True, help="Mensaje a enviar")
    parser.add_argument(
        "-c", "--channel", help="Canal específico (telegram, email, slack, discord)"
    )
    parser.add_argument("-t", "--title", default="Sintonía 3026", help="Título")

    args = parser.parse_args()

    print("[NOTIFICATION-CENTER] Centro de Notificaciones")
    print("=" * 50)

    config = load_config()

    if not config:
        print("[ERROR] No hay canales configurados")
        print("[INFO] Configura variables de entorno:")
        print("   TELEGRAM_BOT_TOKEN, TELEGRAM_CHAT_ID")
        print("   SMTP_HOST, SMTP_PORT, SMTP_USERNAME, SMTP_PASSWORD")
        print("   SLACK_WEBHOOK, DISCORD_WEBHOOK")
        return 1

    message = f"*{args.title}*\n\n{args.message}"

    channels = [args.channel] if args.channel else list(config.keys())

    print(f"[INFO] Enviando a: {', '.join(channels)}")

    results = {}
    for channel in channels:
        if channel not in config:
            print(f"   [WARN] {channel} no configurado")
            continue

        success, result = CHANNELS[channel](message, config[channel])
        results[channel] = (success, result)

        if success:
            print(f"   [OK] {channel}: {result}")
        else:
            print(f"   [ERROR] {channel}: {result}")

    success_count = sum(1 for s, _ in results.values() if s)

    print("\n" + "=" * 50)
    print(f"[OK] Enviado a {success_count}/{len(results)} canales")

    return 0 if success_count > 0 else 1


if __name__ == "__main__":
    sys.exit(main())
