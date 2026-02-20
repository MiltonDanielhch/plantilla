#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
CACHE-MANAGER - Gestor de Caché
Limpia y gestiona caché de Redis, Memcached, Python, Node.
Trigger: Manual o cron
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import subprocess
import sys
from pathlib import Path


def clear_redis():
    try:
        result = subprocess.run(
            ["redis-cli", "FLUSHALL"], capture_output=True, text=True, timeout=10
        )
        if result.returncode == 0:
            print("   [OK] Redis limpiado")
            return True
    except:
        print("   [WARN] Redis no disponible")
    return False


def clear_memcached():
    try:
        result = subprocess.run(
            ["echo", "flush_all", "|", "nc", "localhost", "11211"],
            shell=True,
            capture_output=True,
            text=True,
        )
        print("   [OK] Memcached limpiado")
        return True
    except:
        print("   [WARN] Memcached no disponible")
    return False


def clear_python_cache():
    patterns = ["__pycache__", "*.pyc", "*.pyo", ".pytest_cache", ".mypy_cache"]
    count = 0

    for pattern in patterns:
        for path in Path(".").rglob(pattern):
            try:
                if path.is_file():
                    path.unlink()
                    count += 1
                elif path.is_dir():
                    import shutil

                    shutil.rmtree(path)
                    count += 1
            except:
                pass

    print(f"   [OK] Python cache: {count} elementos")
    return True


def clear_node_cache():
    count = 0

    if Path(".npm").exists():
        try:
            subprocess.run(["npm", "cache", "clean", "--force"], capture_output=True)
            count += 1
        except:
            pass

    if Path("node_modules/.cache").exists():
        import shutil

        shutil.rmtree("node_modules/.cache")
        count += 1

    print(f"   [OK] Node cache limpiado")
    return True


def clear_docker_cache():
    try:
        subprocess.run(["docker", "system", "prune", "-f"], capture_output=True)
        print("   [OK] Docker cache limpiado")
        return True
    except:
        print("   [WARN] Docker no disponible")
    return False


def clear_cloudflare():
    print("   [INFO] Configura CLOUDFLARE_API_KEY para limpiar cache de Cloudflare")
    return False


def get_cache_stats():
    stats = {}

    try:
        result = subprocess.run(
            ["redis-cli", "INFO", "memory"], capture_output=True, text=True, timeout=5
        )
        if result.returncode == 0:
            for line in result.stdout.split("\n"):
                if "used_memory_human" in line:
                    stats["redis"] = line.split(":")[1].strip()
    except:
        pass

    return stats


def main():
    import argparse

    parser = argparse.ArgumentParser(description="Cache Manager")
    parser.add_argument("--redis", action="store_true", help="Limpiar Redis")
    parser.add_argument("--memcached", action="store_true", help="Limpiar Memcached")
    parser.add_argument("--python", action="store_true", help="Limpiar Python cache")
    parser.add_argument("--node", action="store_true", help="Limpiar Node cache")
    parser.add_argument("--docker", action="store_true", help="Limpiar Docker cache")
    parser.add_argument("--all", action="store_true", help="Limpiar todo")
    parser.add_argument("--stats", action="store_true", help="Ver estadísticas")

    args = parser.parse_args()

    print("[CACHE-MANAGER] Gestor de Caché")
    print("=" * 50)

    if args.stats:
        stats = get_cache_stats()
        print("\n[INFO] Estadísticas de caché:")
        for cache, stat in stats.items():
            print(f"   {cache}: {stat}")
        return 0

    if not any(
        [args.redis, args.memcached, args.python, args.node, args.docker, args.all]
    ):
        print("\n[INFO] Uso: cache-manager --[opcion]")
        print("   --redis      Limpiar Redis")
        print("   --python     Limpiar cache Python")
        print("   --node       Limpiar cache Node")
        print("   --docker     Limpiar cache Docker")
        print("   --all        Limpiar todo")
        print("   --stats      Ver estadísticas")
        return 0

    if args.all:
        args.redis = args.python = args.node = args.docker = True

    print("\n[INFO] Limpiando cachés...")

    if args.redis:
        clear_redis()

    if args.memcached:
        clear_memcached()

    if args.python:
        clear_python_cache()

    if args.node:
        clear_node_cache()

    if args.docker:
        clear_docker_cache()

    print("\n[OK] Limpieza completada")
    return 0


if __name__ == "__main__":
    sys.exit(main())
