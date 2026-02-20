#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
POST-MORTEM - Diagnóstico de Crisis
Captura el estado del sistema cuando hay un error 500.
Trigger: Automático cuando se detecta error o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import platform
import psutil
import subprocess
import json
from pathlib import Path
from datetime import datetime
import sys

OUTPUT_DIR = Path("logs/postmortem")
OUTPUT_FILE = OUTPUT_DIR / f"postmortem_{datetime.now().strftime('%Y%m%d_%H%M%S')}.md"


def get_system_info():
    return {
        "hostname": platform.node(),
        "os": f"{platform.system()} {platform.release()}",
        "python": platform.python_version(),
        "cpu_percent": psutil.cpu_percent(interval=1),
        "memory_percent": psutil.virtual_memory().percent,
        "disk_percent": psutil.disk_usage("/").percent,
    }


def get_processes():
    processes = []
    for proc in psutil.process_iter(["pid", "name", "cpu_percent", "memory_percent"]):
        try:
            processes.append(
                {
                    "pid": proc.info["pid"],
                    "name": proc.info["name"],
                    "cpu": proc.info["cpu_percent"],
                    "mem": proc.info["memory_percent"],
                }
            )
        except:
            pass

    return sorted(processes, key=lambda x: x["mem"] if x["mem"] else 0, reverse=True)[
        :20
    ]


def get_recent_logs(lines=100):
    log_dir = Path("logs")
    recent = []

    for log_file in log_dir.glob("*.log"):
        try:
            with open(log_file, "r", encoding="utf-8", errors="ignore") as f:
                file_lines = f.readlines()
                recent.extend([f"## {log_file.name}\n"] + file_lines[-lines:])
        except:
            pass

    return recent[-500:]


def get_env_vars():
    env = {}
    for key in ["DATABASE_URL", "REDIS_URL", "API_KEY", "SECRET", "DEBUG", "NODE_ENV"]:
        if key in os.environ:
            env[key] = (
                "[OCULTO]" if "SECRET" in key or "KEY" in key else os.environ[key]
            )
    return env


def main():
    print("[POST-MORTEM] Diagnóstico de Crisis")
    print("=" * 50)

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    print("[INFO] Recopilando información del sistema...")
    sys_info = get_system_info()

    print("[INFO] Obteniendo procesos...")
    processes = get_processes()

    print("[INFO] Obteniendo logs recientes...")
    logs = get_recent_logs()

    print("[INFO] Obteniendo variables de entorno...")
    env_vars = get_env_vars()

    md_content = f"""# Post-Mortem - Sintonía 3026

**Fecha:** {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}

## Información del Sistema

| Campo | Valor |
|-------|-------|
| Hostname | {sys_info["hostname"]} |
| OS | {sys_info["os"]} |
| Python | {sys_info["python"]} |
| CPU | {sys_info["cpu_percent"]}% |
| Memoria | {sys_info["memory_percent"]}% |
| Disco | {sys_info["disk_percent"]}% |

## Procesos (Top 20 por memoria)

| PID | Nombre | CPU% | Memoria% |
|-----|--------|------|----------|
"""

    for proc in processes:
        md_content += (
            f"| {proc['pid']} | {proc['name']} | {proc['cpu']} | {proc['mem']} |\n"
        )

    md_content += "\n## Variables de Entorno\n\n"
    if env_vars:
        for key, value in env_vars.items():
            md_content += f"- {key}: {value}\n"
    else:
        md_content += "_Ninguna relevante expuesta_\n"

    md_content += "\n## Logs Recientes\n\n"
    md_content += "".join(logs)

    with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
        f.write(md_content)

    print(f"\n[OK] Reporte guardado: {OUTPUT_FILE}")

    if sys_info["memory_percent"] > 90:
        print("[ALERT] ⚠️ Uso de memoria crítico!")
    if sys_info["disk_percent"] > 90:
        print("[ALERT] ⚠️ Disco casi lleno!")

    return 0


if __name__ == "__main__":
    sys.exit(main())
