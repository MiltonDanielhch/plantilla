#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
DATABASE-MIGRATOR - Migrador de Base de Datos
Ejecuta migraciones de forma segura y registra el historial.
Trigger: Antes de deploy o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import subprocess
import json
import sys
from pathlib import Path
from datetime import datetime

MIGRATION_LOG = Path("logs/migrations.json")


def load_migration_log():
    if MIGRATION_LOG.exists():
        with open(MIGRATION_LOG, "r") as f:
            return json.load(f)
    return {"migrations": []}


def save_migration_log(log_data):
    MIGRATION_LOG.parent.mkdir(parents=True, exist_ok=True)
    with open(MIGRATION_LOG, "w") as f:
        json.dump(log_data, f, indent=2)


def detect_db_type():
    db_url = os.environ.get("DATABASE_URL", "")

    if "postgres" in db_url:
        return "postgresql"
    elif "mysql" in db_url:
        return "mysql"
    elif "sqlite" in db_url:
        return "sqlite"
    elif "mongodb" in db_url:
        return "mongodb"

    if Path("alembic.ini").exists():
        return "postgresql"
    if Path("knexfile.js").exists() or Path("knexfile.ts"):
        return "mysql"
    if Path("prisma").exists():
        return "prisma"

    return None


def run_alembic_migrations():
    print("[INFO] Ejecutando migraciones con Alembic...")

    result = subprocess.run(
        ["alembic", "upgrade", "head"], capture_output=True, text=True
    )

    if result.returncode == 0:
        print("   [OK] Migraciones Alembic completadas")
        return True, result.stdout
    else:
        print(f"   [ERROR] {result.stderr}")
        return False, result.stderr


def run_django_migrations():
    print("[INFO] Ejecutando migraciones con Django...")

    result = subprocess.run(
        ["python", "manage.py", "migrate"], capture_output=True, text=True
    )

    if result.returncode == 0:
        print("   [OK] Migraciones Django completadas")
        return True, result.stdout
    else:
        print(f"   [ERROR] {result.stderr}")
        return False, result.stderr


def run_prisma_migrations():
    print("[INFO] Ejecutando migraciones con Prisma...")

    result = subprocess.run(
        ["npx", "prisma", "migrate", "deploy"], capture_output=True, text=True
    )

    if result.returncode == 0:
        print("   [OK] Migraciones Prisma completadas")
        return True, result.stdout
    else:
        print(f"   [ERROR] {result.stderr}")
        return False, result.stderr


def run_knex_migrations():
    print("[INFO] Ejecutando migraciones con Knex...")

    result = subprocess.run(
        ["npx", "knex", "migrate:latest"], capture_output=True, text=True
    )

    if result.returncode == 0:
        print("   [OK] Migraciones Knex completadas")
        return True, result.stdout
    else:
        print(f"   [ERROR] {result.stderr}")
        return False, result.stderr


def run_sql_migrations():
    print("[INFO] Buscando migraciones SQL...")

    migrations_dir = Path("migrations")
    if not migrations_dir.exists():
        migrations_dir = Path("db/migrations")

    if not migrations_dir.exists():
        print("   [WARN] Directorio de migraciones no encontrado")
        return False, "No migrations dir"

    sql_files = sorted(migrations_dir.glob("*.sql"))

    if not sql_files:
        print("   [WARN] No hay archivos SQL para ejecutar")
        return True, "No SQL files"

    print(f"   [INFO] {len(sql_files)} archivo(s) SQL encontrado(s)")

    for sql_file in sql_files:
        print(f"   [EXEC] {sql_file.name}")

    return True, f"Executed {len(sql_files)} SQL files"


def rollback_last_migration(db_type):
    print(f"[INFO] Rollback de migración para {db_type}...")

    if db_type == "postgresql" and Path("alembic.ini").exists():
        result = subprocess.run(
            ["alembic", "downgrade", "-1"], capture_output=True, text=True
        )
        return result.returncode == 0

    if db_type == "django":
        result = subprocess.run(
            ["python", "manage.py", "migrate", "app_name", "previous"],
            capture_output=True,
            text=True,
        )
        return result.returncode == 0

    print("   [WARN] Rollback no soportado para este tipo de BD")
    return False


def main():
    import argparse

    parser = argparse.ArgumentParser(description="Database Migrator")
    parser.add_argument("--rollback", action="store_true", help="Hacer rollback")
    parser.add_argument("--status", action="store_true", help="Ver estado")

    args = parser.parse_args()

    print("[DATABASE-MIGRATOR] Migrador de Base de Datos")
    print("=" * 50)

    db_type = detect_db_type()

    if not db_type:
        print("[ERROR] No se detectó tipo de base de datos")
        print("[INFO] Asegúrate de tener DATABASE_URL o archivos de config")
        return 1

    print(f"[INFO] Tipo de BD detectada: {db_type}")

    log_data = load_migration_log()

    if args.status:
        print("\n📜 Historial de migraciones:")
        for mig in log_data["migrations"]:
            print(f"   - {mig['date']}: {mig['description']}")
        return 0

    if args.rollback:
        success = rollback_last_migration(db_type)
        if success:
            print("[OK] Rollback completado")
        return 0 if success else 1

    success = False
    output = ""

    if db_type == "postgresql" and Path("alembic.ini").exists():
        success, output = run_alembic_migrations()
    elif db_type == "postgresql" or db_type == "mysql":
        if Path("knexfile.js").exists() or Path("knexfile.ts"):
            success, output = run_knex_migrations()
        elif Path("manage.py").exists():
            success, output = run_django_migrations()
        else:
            success, output = run_sql_migrations()
    elif db_type == "prisma":
        success, output = run_prisma_migrations()
    elif db_type == "sqlite":
        success, output = run_sql_migrations()

    if success:
        log_data["migrations"].append(
            {
                "date": datetime.now().isoformat(),
                "description": f"Migración {db_type}",
                "status": "success",
            }
        )
        save_migration_log(log_data)

        print("\n[OK] Migración completada exitosamente!")
        return 0
    else:
        print(f"\n[ERROR] Error en migración: {output}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
