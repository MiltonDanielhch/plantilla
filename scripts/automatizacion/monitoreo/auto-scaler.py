#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
AUTO-SCALER - Autoescalado
Escala servicios automáticamente según la carga.
Trigger: Cron cada minuto o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import time
import sys
import psutil
import requests
import json
import subprocess
from pathlib import Path
from datetime import datetime, timedelta

CONFIG = {
    "cpu_scale_up": int(os.environ.get("CPU_SCALE_UP", "80")),
    "cpu_scale_down": int(os.environ.get("CPU_SCALE_DOWN", "30")),
    "memory_scale_up": int(os.environ.get("MEM_SCALE_UP", "80")),
    "memory_scale_down": int(os.environ.get("MEM_SCALE_DOWN", "30")),
    "min_instances": int(os.environ.get("MIN_INSTANCES", "1")),
    "max_instances": int(os.environ.get("MAX_INSTANCES", "5")),
    "scale_cooldown": int(os.environ.get("SCALE_COOLDOWN", "300")),
}

STATE_FILE = Path("logs/auto_scaler_state.json")


def load_state():
    if STATE_FILE.exists():
        with open(STATE_FILE, "r") as f:
            return json.load(f)
    return {
        "current_instances": CONFIG["min_instances"],
        "last_scale": None,
        "scale_history": [],
    }


def save_state(state):
    STATE_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(STATE_FILE, "w") as f:
        json.dump(state, f, indent=2)


def get_metrics():
    cpu = psutil.cpu_percent(interval=1)
    memory = psutil.virtual_memory().percent

    load_avg = (0, 0, 0)
    try:
        load_avg = psutil.getloadavg()
    except:
        pass

    return {
        "cpu": cpu,
        "memory": memory,
        "load_1m": load_avg[0],
        "load_5m": load_avg[1],
    }


def check_app_health():
    urls = os.environ.get("HEALTH_URLS", "http://localhost:3000/health").split(",")

    for url in urls:
        try:
            response = requests.get(url.strip(), timeout=5)
            if response.status_code >= 500:
                return False
        except:
            return False

    return True


def get_current_instances():
    try:
        result = subprocess.run(
            ["docker", "ps", "--filter", "name=app", "--format", "{{.Names}}"],
            capture_output=True,
            text=True,
            timeout=10,
        )
        count = len([l for l in result.stdout.split("\n") if l.strip()])
        return max(count, CONFIG["min_instances"])
    except:
        return CONFIG["min_instances"]


def scale_up():
    state = load_state()

    if state["current_instances"] >= CONFIG["max_instances"]:
        print("   [INFO] Máximo de instancias alcanzado")
        return False

    new_instances = min(state["current_instances"] + 1, CONFIG["max_instances"])

    print(f"   [SCALE UP] De {state['current_instances']} a {new_instances} instancias")

    try:
        subprocess.run(
            ["docker-compose", "up", "-d", "--scale", f"app={new_instances}"],
            capture_output=True,
            timeout=60,
        )

        state["current_instances"] = new_instances
        state["last_scale"] = datetime.now().isoformat()
        state["scale_history"].append(
            {
                "time": datetime.now().isoformat(),
                "action": "scale_up",
                "instances": new_instances,
            }
        )
        save_state(state)

        notify(f"🔼 AUTO-SCALER: Escalando a {new_instances} instancias")
        return True
    except Exception as e:
        print(f"   [ERROR] {e}")
        return False


def scale_down():
    state = load_state()

    if state["current_instances"] <= CONFIG["min_instances"]:
        print("   [INFO] Mínimo de instancias alcanzado")
        return False

    new_instances = max(state["current_instances"] - 1, CONFIG["min_instances"])

    print(
        f"   [SCALE DOWN] De {state['current_instances']} a {new_instances} instancias"
    )

    try:
        subprocess.run(
            ["docker-compose", "up", "-d", "--scale", f"app={new_instances}"],
            capture_output=True,
            timeout=60,
        )

        state["current_instances"] = new_instances
        state["last_scale"] = datetime.now().isoformat()
        state["scale_history"].append(
            {
                "time": datetime.now().isoformat(),
                "action": "scale_down",
                "instances": new_instances,
            }
        )
        save_state(state)

        notify(f"🔽 AUTO-SCALER: Reduciendo a {new_instances} instancias")
        return True
    except Exception as e:
        print(f"   [ERROR] {e}")
        return False


def notify(message):
    print(f"   [NOTIFY] {message}")

    if "TELEGRAM_BOT_TOKEN" in os.environ and "TELEGRAM_CHAT_ID" in os.environ:
        try:
            requests.post(
                f"https://api.telegram.org/bot{os.environ['TELEGRAM_BOT_TOKEN']}/sendMessage",
                json={
                    "chat_id": os.environ["TELEGRAM_CHAT_ID"],
                    "text": message,
                    "parse_mode": "Markdown",
                },
            )
        except:
            pass


def should_scale(metrics):
    cpu = metrics["cpu"]
    memory = metrics["memory"]

    if cpu > CONFIG["cpu_scale_up"] or memory > CONFIG["memory_scale_up"]:
        return "up"

    if cpu < CONFIG["cpu_scale_down"] and memory < CONFIG["memory_scale_down"]:
        return "down"

    return None


def can_scale():
    state = load_state()

    if not state["last_scale"]:
        return True

    last_scale = datetime.fromisoformat(state["last_scale"])
    cooldown_end = last_scale + timedelta(seconds=CONFIG["scale_cooldown"])

    return datetime.now() > cooldown_end


def main():
    import argparse

    parser = argparse.ArgumentParser(description="Auto-Scaler")
    parser.add_argument("--status", action="store_true", help="Ver estado")
    parser.add_argument("--scale-up", action="store_true", help="Escalar arriba")
    parser.add_argument("--scale-down", action="store_true", help="Escalar abajo")

    args = parser.parse_args()

    print("[AUTO-SCALER] Autoescalado de Servicios")
    print("=" * 50)

    if args.status:
        state = load_state()
        metrics = get_metrics()

        print(f"\n📊 Estado actual:")
        print(f"   Instancias: {state['current_instances']}")
        print(f"   CPU: {metrics['cpu']}%")
        print(f"   Memoria: {metrics['memory']}%")

        if state["last_scale"]:
            print(f"   Último scale: {state['last_scale']}")

        return 0

    if args.scale_up:
        return 0 if scale_up() else 1

    if args.scale_down:
        return 0 if scale_down() else 1

    if not check_app_health():
        print("[ERROR] La app no está saludable")
        return 1

    metrics = get_metrics()
    print(f"[INFO] Métricas: CPU={metrics['cpu']}%, Mem={metrics['memory']}%")

    action = should_scale(metrics)

    if not action:
        print("[OK] No es necesario escalar")
        return 0

    if not can_scale():
        print("[INFO] En cooldown, no se puede escalar todavía")
        return 0

    if action == "up":
        return 0 if scale_up() else 1
    elif action == "down":
        return 0 if scale_down() else 1


if __name__ == "__main__":
    sys.exit(main())
