#!/usr/bin/env python3
"""
DEEP-WORK-LOGGER - Cronómetro de Enfoque
Registra tiempo de trabajo por archivo y genera reportes.
Trigger: Ejecución manual o integración con editor (Vim, VSCode)
"""

import os
import json
import sys
import codecs
import time
from datetime import datetime, date
from pathlib import Path

# Fix for Windows UTF-8 encoding
if sys.platform == "win32":
    sys.stdout = codecs.getwriter("utf-8")(sys.stdout.buffer, "strict")
    sys.stderr = codecs.getwriter("utf-8")(sys.stderr.buffer, "strict")

LOG_FILE = Path.home() / ".sintonía" / "deep_work_log.json"


def load_log():
    if LOG_FILE.exists():
        with open(LOG_FILE, "r") as f:
            return json.load(f)
    return {}


def save_log(log_data):
    LOG_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(LOG_FILE, "w") as f:
        json.dump(log_data, f, indent=2)


def start_session():
    print("🧠 DEEP-WORK-LOGGER - Sesión iniciada")
    print("=" * 50)
    print(f"📅 Fecha: {date.today()}")
    print("💡 Presiona Ctrl+C para terminar la sesión")
    print("🎯 Working...")

    start_time = time.time()
    current_file = None

    try:
        while True:
            time.sleep(10)

            active_file = get_active_file()
            if active_file != current_file:
                if current_file:
                    log = load_log()
                    today = str(date.today())
                    if today not in log:
                        log[today] = {}
                    if current_file not in log[today]:
                        log[today][current_file] = 0

                    elapsed = time.time() - start_time
                    log[today][current_file] += elapsed
                    save_log(log)

                    print(f"\n⏱️ Registrado: {current_file} ({elapsed / 60:.1f} min)")

                current_file = active_file
                start_time = time.time()
                if active_file:
                    print(f"📝 Trabajando en: {active_file}")

    except KeyboardInterrupt:
        if current_file:
            elapsed = time.time() - start_time
            log = load_log()
            today = str(date.today())
            if today not in log:
                log[today] = {}
            if current_file not in log[today]:
                log[today][current_file] = 0
            log[today][current_file] += elapsed
            save_log(log)
            print(f"\n⏱️ Último registro: {current_file} ({elapsed / 60:.1f} min)")

        print("\n✅ Sesión guardada")


def get_active_file():
    try:
        if sys.platform == "win32":
            import subprocess

            result = subprocess.run(
                [
                    "powershell",
                    "-c",
                    "(Get-Process | Where-Object {$_.MainWindowTitle} | Select-Object -First 1).MainWindowTitle",
                ],
                capture_output=True,
                text=True,
            )
            return result.stdout.strip() or "Unknown"
        else:
            return "Unix detect"
    except:
        return "Unknown"


def show_report(days=7):
    log = load_log()
    print("📊 REPORTE DE ENFOQUE PROFUNDO")
    print("=" * 50)

    today = date.today()
    total_time = {}

    for i in range(days):
        day = today.replace(day=today.day - i)
        day_str = str(day)

        if day_str in log:
            day_total = sum(log[day_str].values())
            total_time[day_str] = day_total

            print(f"\n📅 {day.strftime('%A %d/%m/%Y')}")
            print(f"   ⏱️  Total: {day_total / 3600:.2f} horas")

            sorted_files = sorted(
                log[day_str].items(), key=lambda x: x[1], reverse=True
            )
            for filepath, secs in sorted_files[:5]:
                print(f"   • {Path(filepath).name}: {secs / 60:.1f} min")

    print("\n" + "=" * 50)
    print("📈 RESUMEN SEMANAL")
    week_total = sum(total_time.values())
    print(f"   Total: {week_total / 3600:.2f} horas")
    print(f"   Promedio diario: {week_total / (days * 3600):.2f} horas")


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--report":
        days = int(sys.argv[2]) if len(sys.argv) > 2 else 7
        show_report(days)
        return 0

    start_session()
    return 0


if __name__ == "__main__":
    sys.exit(main())
