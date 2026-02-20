#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
SSL-RENEWER - Renovador de Certificados SSL
Renueva automáticamente certificados Let's Encrypt antes de que expiren.
Trigger: Cron semanal o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import subprocess
import json
from pathlib import Path
from datetime import datetime, timedelta
import sys

CERT_DIR = Path("/etc/letsencrypt/live")
DAYS_BEFORE_RENEWAL = int(os.environ.get("DAYS_BEFORE_RENEWAL", "7"))
NOTIFY_DAYS = int(os.environ.get("NOTIFY_DAYS", "14"))


def get_cert_expiry(cert_path):
    try:
        result = subprocess.run(
            ["openssl", "x509", "-enddate", "-noout", "-in", cert_path],
            capture_output=True,
            text=True,
            timeout=10,
        )

        if result.returncode == 0:
            date_str = result.stdout.replace("notAfter=", "").strip()
            from datetime import datetime

            expiry = datetime.strptime(date_str, "%b %d %H:%M:%S %Y %Z")
            return expiry
    except:
        pass
    return None


def check_cert(domain):
    cert_file = CERT_DIR / domain / "fullchain.pem"

    if not cert_file.exists():
        return {"status": "missing", "days_left": 0}

    expiry = get_cert_expiry(str(cert_file))

    if not expiry:
        return {"status": "unknown", "days_left": 0}

    days_left = (expiry - datetime.now()).days

    if days_left < 0:
        status = "expired"
    elif days_left <= DAYS_BEFORE_RENEWAL:
        status = "renew"
    elif days_left <= NOTIFY_DAYS:
        status = "expiring_soon"
    else:
        status = "valid"

    return {
        "status": status,
        "days_left": days_left,
        "expiry": expiry.strftime("%Y-%m-%d"),
        "domain": domain,
    }


def renew_cert(domain):
    print(f"  [RENEW] Renovando certificado para {domain}...")

    try:
        result = subprocess.run(
            ["certbot", "renew", "--cert-name", domain, "--force-renewal"],
            capture_output=True,
            text=True,
            timeout=300,
        )

        if result.returncode == 0:
            print(f"  [OK] Certificado renovado para {domain}")

            if "nginx" in result.stdout or "apache" in result.stdout:
                subprocess.run(["systemctl", "reload", "nginx"], capture_output=True)
                print(f"  [OK] Servidor recargado")

            return True
        else:
            print(f"  [ERROR] {result.stderr}")
            return False
    except Exception as e:
        print(f"  [ERROR] {e}")
        return False


def notify_expiry(domain, days_left):
    msg = f"🔔 *AVISO SSL*\n\nDominio: {domain}\nDías restantes: {days_left}\nRenueva pronto!"

    if "TELEGRAM_BOT_TOKEN" in os.environ and "TELEGRAM_CHAT_ID" in os.environ:
        import requests

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
    print("[SSL-RENEWER] Renovador de Certificados SSL")
    print("=" * 50)

    domains = os.environ.get("SSL_DOMAINS", "").split(",")

    if not domains or domains == [""]:
        print("[INFO] Buscando certificados en:", CERT_DIR)

        if CERT_DIR.exists():
            domains = [d.name for d in CERT_DIR.iterdir() if d.is_dir()]
        else:
            print("[WARNING] Directorio de certificados no encontrado")
            print("[INFO] Configura SSL_DOMAINS o certbot debe estar instalado")
            domains = []

    print(f"[INFO] Dominios a verificar: {len(domains)}\n")

    results = []

    for domain in domains:
        domain = domain.strip()
        if not domain:
            continue

        print(f"📄 {domain}:")
        result = check_cert(domain)

        if result["status"] == "valid":
            print(f"   [OK] Válido por {result['days_left']} días")
        elif result["status"] == "expiring_soon":
            print(f"   [WARN] Expira en {result['days_left']} días")
            notify_expiry(domain, result["days_left"])
        elif result["status"] == "renew":
            print(f"   [RENEW] Expira en {result['days_left']} días - Renovando...")
            if renew_cert(domain):
                result["status"] = "renewed"
        elif result["status"] == "expired":
            print(f"   [ERROR] Certificado expirado!")
            notify_expiry(domain, 0)
        elif result["status"] == "missing":
            print(f"   [WARN] Certificado no encontrado")

        results.append(result)

    print("\n" + "=" * 50)
    print("📊 RESUMEN:")

    valid = sum(1 for r in results if r["status"] == "valid")
    renew = sum(1 for r in results if r["status"] in ["renew", "renewed"])
    expired = sum(1 for r in results if r["status"] == "expired")

    print(f"  Válidos: {valid}")
    print(f"  Renovados: {renew}")
    print(f"  Expirados: {expired}")

    if expired > 0:
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
