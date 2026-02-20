#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
DATABASE-HEALTH - Verificador de Salud de BD
Verifica conexiones, tamaño, rendimiento de la base de datos.
Trigger: Cron o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import psycopg2
import pymysql
from pathlib import Path
from datetime import datetime


def get_postgres_health(db_url):
    try:
        conn = psycopg2.connect(db_url)
        cur = conn.cursor()

        cur.execute("SELECT version();")
        version = cur.fetchone()[0]

        cur.execute("SELECT pg_database_size(current_database());")
        size = cur.fetchone()[0]

        cur.execute("SELECT count(*) FROM pg_stat_activity;")
        connections = cur.fetchone()[0]

        cur.execute(
            "SELECT sum(numbackends) FROM pg_stat_database WHERE datname = current_database();"
        )
        active_conn = cur.fetchone()[0] or 0

        conn.close()

        return {
            "status": "ok",
            "version": version[:50],
            "size_mb": round(size / (1024 * 1024), 2),
            "connections": connections,
            "active_connections": active_conn,
        }
    except Exception as e:
        return {"status": "error", "message": str(e)}


def get_mysql_health(db_url):
    try:
        conn = pymysql.connect(
            host=db_url.get("host", "localhost"),
            user=db_url.get("user", "root"),
            password=db_url.get("password", ""),
            database=db_url.get("database", ""),
        )
        cur = conn.cursor()

        cur.execute("SELECT VERSION();")
        version = cur.fetchone()[0]

        cur.execute("SHOW TABLE STATUS;")
        tables = cur.fetchall()
        size = sum(t[6] for t in tables if t[6])

        cur.execute("SHOW STATUS LIKE 'Threads_connected';")
        connections = cur.fetchone()[1]

        conn.close()

        return {
            "status": "ok",
            "version": version[:50],
            "size_mb": round(size / (1024 * 1024), 2),
            "connections": connections,
        }
    except Exception as e:
        return {"status": "error", "message": str(e)}


def parse_postgres_url(url):
    return url


def main():
    print("[DATABASE-HEALTH] Verificador de Salud de BD")
    print("=" * 50)

    db_url = os.environ.get("DATABASE_URL", "")

    if not db_url:
        print("[ERROR] DATABASE_URL no configurado")
        return 1

    if "postgres" in db_url:
        health = get_postgres_health(db_url)
    elif "mysql" in db_url:
        health = get_mysql_health({"host": "localhost"})
    else:
        print("[ERROR] Base de datos no soportada")
        return 1

    print(f"\n[INFO] Estado: {health.get('status', 'unknown')}")

    if health["status"] == "ok":
        print(f"   Versión: {health['version']}")
        print(f"   Tamaño: {health['size_mb']} MB")
        print(f"   Conexiones: {health['connections']}")
        if "active_connections" in health:
            print(f"   Conexiones activas: {health['active_connections']}")
        print("\n[OK] Base de datos saludable")
        return 0
    else:
        print(f"   Error: {health.get('message', 'Unknown')}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
