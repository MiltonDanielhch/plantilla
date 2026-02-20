#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
SHIELD - Guardian de Secretos
Escanea el código en busca de API Keys, passwords, tokens y otros secretos expuestos.
Trigger: Git Pre-commit hook o ejecución manual con `just shield`
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import re
from pathlib import Path

SECRET_PATTERNS = {
    "AWS Access Key": r"(?:A3T[A-Z0-9]|AKIA|AGPA|AIDA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{16}",
    "AWS Secret Key": r'(?i)(?:aws_secret_access_key|aws_secret_key)\s*=\s*[\'"][A-Za-z0-9/+=]{40}[\'"]',
    "GitHub Token": r"(?:ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9_]{36,255}",
    "Generic API Key": r'(?i)(?:api[_-]?key|apikey)\s*[:=]\s*[\'"][A-Za-z0-9]{16,}[\'"]',
    "Generic Secret": r'(?i)(?:password|passwd|pwd|secret)\s*[:=]\s*[\'"][^\'"]+[\'"]',
    "Private Key": r"-----BEGIN (?:RSA |DSA |EC |OPENSSH )?PRIVATE KEY-----",
    "JWT Token": r"eyJ[A-Za-z0-9-_]+\.eyJ[A-Za-z0-9-_]+\.[A-Za-z0-9-_]+",
    "Slack Token": r"xox[baprs]-[0-9]{10,13}-[0-9]{10,13}[a-zA-Z0-9-]*",
    "Discord Token": r"[MN][A-Za-z\d]{23,}\.[\w-]{6}\.[\w-]{27}",
    "Database URL": r'(?i)(?:mysql|postgres|mongodb)://[^\s\'"]+',
}

EXCLUDE_DIRS = {
    ".git",
    "node_modules",
    "target",
    "__pycache__",
    ".venv",
    "venv",
    "dist",
    "build",
}
EXCLUDE_FILES = {".env", ".env.local", ".env.production"}


def scan_file(filepath):
    """Escanea un archivo en busca de secretos."""
    secrets_found = []
    try:
        with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
            for line_num, line in enumerate(f, 1):
                for secret_type, pattern in SECRET_PATTERNS.items():
                    if re.search(pattern, line):
                        secrets_found.append(
                            {
                                "file": filepath,
                                "line": line_num,
                                "type": secret_type,
                                "content": line.strip()[:80],
                            }
                        )
    except Exception as e:
        print(f"⚠️ Error leyendo {filepath}: {e}")
    return secrets_found


def scan_directory(root_dir):
    """Escanea recursivamente un directorio."""
    all_secrets = []
    root_path = Path(root_dir)

    for filepath in root_path.rglob("*"):
        if filepath.is_file():
            if any(excluded in filepath.parts for excluded in EXCLUDE_DIRS):
                continue
            if filepath.name in EXCLUDE_FILES:
                continue

            secrets = scan_file(str(filepath))
            all_secrets.extend(secrets)

    return all_secrets


def main():
    print("[SHIELD] Guardian de Secretos")
    print("=" * 50)

    project_root = os.getcwd()
    print(f"[OK] Escaneando: {project_root}\n")

    secrets = scan_directory(project_root)

    if not secrets:
        print("[OK] Ningun secreto detectado! Tu codigo esta protegido.")
        return 0

    print(f"[ALERTA] Se detectaron {len(secrets)} posible(s) secreto(s):\n")
    for i, secret in enumerate(secrets, 1):
        print(f"{i}. [{secret['type']}]")
        print(f"   {secret['file']}:{secret['line']}")
        print(f"   {secret['content']}")
        print()

    print(
        "[ERROR] RECOMENDACION: Elimina o enmascara estos secretos antes de hacer commit."
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
