#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
DATABASE-BACKUP - Backup de Base de Datos
Hace backup de la base de datos y optionally lo sube a la nube.
Trigger: Cron diario (0 3 * * *)
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import subprocess
import shutil
from pathlib import Path
from datetime import datetime
import sys

BACKUP_DIR = Path("backups/database")
DROPBOX_UPLOAD = os.environ.get("DROPBOX_TOKEN", "") != ""


def get_db_config():
    db_url = os.environ.get("DATABASE_URL", "")

    if "postgres" in db_url:
        return {"type": "postgresql", "url": db_url}
    elif "mysql" in db_url:
        return {"type": "mysql", "url": db_url}
    elif "mongodb" in db_url:
        return {"type": "mongodb", "url": db_url}

    return None


def backup_postgres(config):
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    filename = f"db_backup_{timestamp}.sql"
    filepath = BACKUP_DIR / filename

    BACKUP_DIR.mkdir(parents=True, exist_ok=True)

    url = config["url"]
    user = os.environ.get("PGUSER", "postgres")
    host = os.environ.get("PGHOST", "localhost")
    dbname = os.environ.get("PGDATABASE", "app")

    cmd = f"pg_dump -U {user} -h {host} -d {dbname} -f {filepath}"

    env = os.environ.copy()
    if "PGPASSWORD" in os.environ:
        env["PGPASSWORD"] = os.environ["PGPASSWORD"]

    result = subprocess.run(cmd, shell=True, env=env, capture_output=True)

    if result.returncode == 0:
        size = filepath.stat().st_size
        print(f"   [OK] Backup guardado: {filepath}")
        print(f"   [OK] Tamaño: {size / (1024 * 1024):.2f} MB")
        return filepath
    else:
        print(f"   [ERROR] {result.stderr.decode()}")
        return None


def backup_mysql(config):
    timestamp = datetime.now().strftime("%Y%m_%d_%H%M%S")
    filename = f"db_backup_{timestamp}.sql"
    filepath = BACKUP_DIR / filename

    BACKUP_DIR.mkdir(parents=True, exist_ok=True)

    user = os.environ.get("MYSQL_USER", "root")
    password = os.environ.get("MYSQL_PASSWORD", "")
    dbname = os.environ.get("MYSQL_DATABASE", "app")

    cmd = f"mysqldump -u {user}"
    if password:
        cmd += f" -p{password}"
    cmd += f" {dbname} > {filepath}"

    result = subprocess.run(cmd, shell=True, capture_output=True)

    if result.returncode == 0:
        size = filepath.stat().st_size
        print(f"   [OK] Backup guardado: {filepath}")
        return filepath
    else:
        print(f"   [ERROR] {result.stderr.decode()}")
        return None


def upload_to_cloud(filepath):
    if not DROPBOX_UPLOAD:
        print("   [INFO] No hay token de Dropbox configurado")
        return False

    try:
        import requests

        url = "https://content.dropboxapi.com/2/files/upload"
        headers = {
            "Authorization": f"Bearer {os.environ['DROPBOX_TOKEN']}",
            "Dropbox-API-Arg": '{"path":"/backups/'
            + filepath.name
            + '","mode":"add","autorename":true}',
            "Content-Type": "application/octet-stream",
        }

        with open(filepath, "rb") as f:
            data = f.read()

        response = requests.post(url, headers=headers, data=data)

        if response.status_code == 200:
            print(f"   [OK] Subido a Dropbox")
            return True
        else:
            print(f"   [ERROR] Dropbox: {response.status_code}")
            return False
    except ImportError:
        print("   [WARN] Requiere: pip install requests")
        return False


def compress_backup(filepath):
    import gzip

    compressed = filepath.with_suffix(".sql.gz")

    with open(filepath, "rb") as f_in:
        with gzip.open(compressed, "wb") as f_out:
            f_out.write(f_in.read())

    original_size = filepath.stat().st_size
    compressed_size = compressed.stat().st_size

    print(
        f"   [OK] Comprimido: {original_size / (1024 * 1024):.2f}MB -> {compressed_size / (1024 * 1024):.2f}MB"
    )

    filepath.unlink()

    return compressed


def main():
    print("[DATABASE-BACKUP] Backup de Base de Datos")
    print("=" * 50)

    config = get_db_config()

    if not config:
        print("[ERROR] No se detectó configuración de base de datos")
        print("[INFO] Configura DATABASE_URL en .env")
        return 1

    print(f"[INFO] Tipo de BD: {config['type']}")

    if config["type"] == "postgresql":
        filepath = backup_postgres(config)
    elif config["type"] == "mysql":
        filepath = backup_mysql(config)
    else:
        print(f"[ERROR] Tipo de BD no soportado: {config['type']}")
        return 1

    if not filepath:
        return 1

    compressed = compress_backup(filepath)
    upload_to_cloud(compressed)

    print("\n[OK] Backup completado!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
