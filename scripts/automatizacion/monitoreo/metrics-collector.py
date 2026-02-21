#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
METRICS-COLLECTOR - Recolector de Métricas
Recolecta métricas del sistema y las guarda en formato Prometheus/InfluxDB.
Trigger: Cron cada 30 segundos o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import time
import psutil
import json
from pathlib import Path
from datetime import datetime

OUTPUT_FILE = Path("logs/metrics.json")
PROMETHEUS_FILE = Path("logs/metrics.prom")


def collect_cpu_metrics():
    return {
        "cpu_percent": psutil.cpu_percent(interval=1),
        "cpu_count": psutil.cpu_count(),
        "cpu_freq": psutil.cpu_freq().current if psutil.cpu_freq() else 0,
        "load_avg": list(psutil.getloadavg())
        if hasattr(psutil, "getloadavg")
        else [0, 0, 0],
    }


def collect_memory_metrics():
    mem = psutil.virtual_memory()
    swap = psutil.swap_memory()
    return {
        "memory_total": mem.total,
        "memory_available": mem.available,
        "memory_percent": mem.percent,
        "memory_used": mem.used,
        "swap_total": swap.total,
        "swap_percent": swap.percent,
    }


def collect_disk_metrics():
    partitions = []
    for partition in psutil.disk_partitions():
        try:
            usage = psutil.disk_usage(partition.mountpoint)
            partitions.append(
                {
                    "device": partition.device,
                    "mountpoint": partition.mountpoint,
                    "fstype": partition.fstype,
                    "total": usage.total,
                    "used": usage.used,
                    "free": usage.free,
                    "percent": usage.percent,
                }
            )
        except:
            pass
    return partitions


def collect_network_metrics():
    net = psutil.net_io_counters()
    return {
        "bytes_sent": net.bytes_sent,
        "bytes_recv": net.bytes_recv,
        "packets_sent": net.packets_sent,
        "packets_recv": net.packets_recv,
        "errin": net.errin,
        "errout": net.errout,
    }


def collect_process_metrics():
    processes = []
    for proc in psutil.process_iter(["pid", "name", "cpu_percent", "memory_percent"]):
        try:
            processes.append(
                {
                    "pid": proc.info["pid"],
                    "name": proc.info["name"],
                    "cpu": proc.info["cpu_percent"],
                    "memory": proc.info["memory_percent"],
                }
            )
        except:
            pass

    processes.sort(key=lambda x: x["memory"] or 0, reverse=True)
    return processes[:10]


def generate_prometheus_format(metrics):
    lines = []
    timestamp = int(time.time() * 1000)

    lines.append(f"# HELP sintonia_cpu_percent CPU usage percentage")
    lines.append(f"# TYPE sintonia_cpu_percent gauge")
    lines.append(f"sintonia_cpu_percent {metrics['cpu']['cpu_percent']} {timestamp}")

    lines.append(f"# HELP sintonia_memory_percent Memory usage percentage")
    lines.append(f"# TYPE sintonia_memory_percent gauge")
    lines.append(
        f"sintonia_memory_percent {metrics['memory']['memory_percent']} {timestamp}"
    )

    if metrics["disk"]:
        disk = metrics["disk"][0]
        lines.append(f"# HELP sintonia_disk_percent Disk usage percentage")
        lines.append(f"# TYPE sintonia_disk_percent gauge")
        lines.append(f"sintonia_disk_percent {disk['percent']} {timestamp}")

    for disk in metrics["disk"]:
        mountpoint = disk["mountpoint"].replace("/", "_").replace(".", "_")
        lines.append(
            f'sintonia_disk_percent{{mountpoint="{disk["mountpoint"]}"}} {disk["percent"]} {timestamp}'
        )

    lines.append(f"# HELP sintonia_network_bytes Network I/O bytes")
    lines.append(f"# TYPE sintonia_network_bytes counter")
    lines.append(
        f"sintonia_network_bytes_sent {metrics['network']['bytes_sent']} {timestamp}"
    )
    lines.append(
        f"sintonia_network_bytes_recv {metrics['network']['bytes_recv']} {timestamp}"
    )

    return "\n".join(lines)


def main():
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--prometheus", action="store_true", help="Formato Prometheus")
    parser.add_argument("--stream", action="store_true", help="Modo streaming")
    args = parser.parse_args()

    if args.stream:
        print("[STREAM] Recolectando métricas cada 5 segundos... (Ctrl+C para salir)")
        try:
            while True:
                metrics = {
                    "timestamp": datetime.now().isoformat(),
                    "cpu": collect_cpu_metrics(),
                    "memory": collect_memory_metrics(),
                    "disk": collect_disk_metrics(),
                    "network": collect_network_metrics(),
                }

                print(
                    f"\n[{datetime.now().strftime('%H:%M:%S')}] "
                    f"CPU: {metrics['cpu']['cpu_percent']}% | "
                    f"Mem: {metrics['memory']['memory_percent']}% | "
                    f"Disk: {metrics['disk'][0]['percent'] if metrics['disk'] else 0}%"
                )

                with open(OUTPUT_FILE, "w") as f:
                    json.dump(metrics, f, indent=2)

                with open(PROMETHEUS_FILE, "w") as f:
                    f.write(generate_prometheus_format(metrics))

                time.sleep(5)
        except KeyboardInterrupt:
            print("\n[OK] Streaming detenido")
        return 0

    print("[METRICS-COLLECTOR] Recolector de Métricas")
    print("=" * 50)

    metrics = {
        "timestamp": datetime.now().isoformat(),
        "cpu": collect_cpu_metrics(),
        "memory": collect_memory_metrics(),
        "disk": collect_disk_metrics(),
        "network": collect_network_metrics(),
        "top_processes": collect_process_metrics(),
    }

    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)

    with open(OUTPUT_FILE, "w") as f:
        json.dump(metrics, f, indent=2)

    with open(PROMETHEUS_FILE, "w") as f:
        f.write(generate_prometheus_format(metrics))

    print(f"\n[OK] Métricas guardadas:")
    print(f"   JSON: {OUTPUT_FILE}")
    print(f"   Prometheus: {PROMETHEUS_FILE}")

    print(f"\n[INFO] Resumen:")
    print(f"   CPU: {metrics['cpu']['cpu_percent']}%")
    print(f"   Memoria: {metrics['memory']['memory_percent']}%")
    print(f"   Disco: {metrics['disk'][0]['percent'] if metrics['disk'] else 'N/A'}%")

    return 0


if __name__ == "__main__":
    sys.exit(main())
