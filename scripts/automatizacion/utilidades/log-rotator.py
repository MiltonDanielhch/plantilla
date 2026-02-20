#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
LOG-ROTATOR - Analizador de Logs
Resume los logs y genera estadísticas de errores.
Trigger: Diario o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import re
from pathlib import Path
from datetime import datetime, timedelta
from collections import Counter, defaultdict
import gzip

LOG_DIR = Path("logs")
OUTPUT_FILE = Path("logs/log_analysis.md")

ERROR_PATTERNS = [
    (r"ERROR", "ERROR"),
    (r"WARN(?:ING)?", "WARNING"),
    (r"FATAL", "FATAL"),
    (r"Exception", "EXCEPTION"),
    (r"404", "NOT_FOUND"),
    (r"500", "SERVER_ERROR"),
    (r"timeout", "TIMEOUT"),
    (r"connection refused", "CONNECTION_REFUSED"),
    (r"out of memory", "OOM"),
]


def parse_log_line(line, date_pattern):
    for pattern, error_type in ERROR_PATTERNS:
        if re.search(pattern, line, re.IGNORECASE):
            return error_type
    return None


def analyze_log_file(filepath):
    errors = Counter()
    total_lines = 0

    try:
        if filepath.suffix == ".gz":
            with gzip.open(filepath, "rt", encoding="utf-8", errors="ignore") as f:
                lines = f.readlines()
        else:
            with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
                lines = f.readlines()

        for line in lines:
            total_lines += 1
            error_type = parse_log_line(line, None)
            if error_type:
                errors[error_type] += 1

    except Exception as e:
        print(f"   [WARNING] Error reading {filepath}: {e}")

    return total_lines, errors


def main():
    print("[LOG-ROTATOR] Analizador de Logs")
    print("=" * 50)

    if not LOG_DIR.exists():
        print("[ERROR] Directorio de logs no existe")
        return 1

    all_errors = Counter()
    total_lines = 0
    files_analyzed = 0

    print(f"[INFO] Analizando logs en: {LOG_DIR}")

    for log_file in LOG_DIR.rglob("*.log"):
        files_analyzed += 1
        lines, errors = analyze_log_file(log_file)
        total_lines += lines
        all_errors.update(errors)

        if errors:
            print(f"   {log_file.name}: {sum(errors.values())} errores")

    for log_file in LOG_DIR.rglob("*.log.gz"):
        files_analyzed += 1
        lines, errors = analyze_log_file(log_file)
        total_lines += lines
        all_errors.update(errors)

    print(f"\n[INFO] Archivos analizados: {files_analyzed}")
    print(f"[INFO] Total de líneas: {total_lines}")

    if not all_errors:
        print("[OK] No se detectaron errores!")

        with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
            f.write(f"# Análisis de Logs - Sintonía 3026\n\n")
            f.write(f"**Fecha:** {datetime.now().strftime('%Y-%m-%d %H:%M')}\n\n")
            f.write("## Resumen\n\n")
            f.write(f"- Archivos analizados: {files_analyzed}\n")
            f.write(f"- Total de líneas: {total_lines}\n")
            f.write("- Errores encontrados: 0\n\n")
            f.write("✅ **¡Todo limpio!**\n")

        print(f"[OK] Reporte guardado: {OUTPUT_FILE}")
        return 0

    print(f"\n[ERROR] Errores encontrados: {sum(all_errors.values())}")
    print("\n📊 Por tipo:")
    for error_type, count in all_errors.most_common(10):
        print(f"   {error_type}: {count}")

    with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
        f.write(f"# Análisis de Logs - Sintonía 3026\n\n")
        f.write(f"**Fecha:** {datetime.now().strftime('%Y-%m-%d %H:%M')}\n\n")
        f.write("## Resumen\n\n")
        f.write(f"- Archivos analizados: {files_analyzed}\n")
        f.write(f"- Total de líneas: {total_lines}\n")
        f.write(f"- Errores encontrados: {sum(all_errors.values())}\n\n")
        f.write("## Detalles\n\n")
        f.write("| Tipo | Cantidad |\n")
        f.write("|------|----------|\n")
        for error_type, count in all_errors.most_common():
            f.write(f"| {error_type} | {count} |\n")
        f.write("\n## Recomendaciones\n\n")

        if "SERVER_ERROR" in all_errors:
            f.write("- ⚠️ Errores 500 detectados. Revisa los logs del servidor.\n")
        if "OOM" in all_errors:
            f.write(
                "- 🚨 Errores de memoria. Considera aumentar RAM o optimizar código.\n"
            )
        if "TIMEOUT" in all_errors:
            f.write(
                "- ⏱️ Timeouts detectados. Revisa la latencia de la base de datos.\n"
            )

    print(f"\n[OK] Reporte guardado: {OUTPUT_FILE}")
    return 1


if __name__ == "__main__":
    sys.exit(main())
