#!/usr/bin/env python3
"""
THE-TRANSFORMER - Conversor de Formatos
Convierte datos entre formatos: CSV <-> JSON, JSON <-> YAML, etc.
Trigger: Ejecución manual cuando necesitas convertir archivos
"""

import json
import csv
import sys
import codecs
import yaml
from pathlib import Path

# Fix for Windows UTF-8 encoding
if sys.platform == "win32":
    sys.stdout = codecs.getwriter("utf-8")(sys.stdout.buffer, "strict")
    sys.stderr = codecs.getwriter("utf-8")(sys.stderr.buffer, "strict")


def csv_to_json(csv_path):
    data = []
    with open(csv_path, "r", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        for row in reader:
            data.append(row)

    json_path = csv_path.replace(".csv", ".json")
    with open(json_path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)

    return json_path, data


def json_to_csv(json_path):
    with open(json_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    if not data:
        return None, 0

    csv_path = json_path.replace(".json", ".csv")
    headers = list(data[0].keys()) if isinstance(data[0], dict) else []

    with open(csv_path, "w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=headers)
        writer.writeheader()
        for row in data:
            if isinstance(row, dict):
                writer.writerow(row)

    return csv_path, len(data)


def json_to_yaml(json_path):
    with open(json_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    yaml_path = json_path.replace(".json", ".yaml")
    with open(yaml_path, "w", encoding="utf-8") as f:
        yaml.dump(data, f, default_flow_style=False, allow_unicode=True)

    return yaml_path, data


def yaml_to_json(yaml_path):
    with open(yaml_path, "r", encoding="utf-8") as f:
        data = yaml.safe_load(f)

    json_path = yaml_path.replace(".yaml", ".json").replace(".yml", ".json")
    with open(json_path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)

    return json_path, data


def detect_conversion(input_path):
    input_ext = Path(input_path).suffix.lower()

    conversions = {
        (".csv", ".json"): csv_to_json,
        (".json", ".csv"): json_to_csv,
        (".json", ".yaml"): json_to_yaml,
        (".json", ".yml"): json_to_yaml,
        (".yaml", ".json"): yaml_to_json,
        (".yml", ".json"): yaml_to_json,
    }

    return conversions.get((input_ext, None))


def main():
    print("🔄 THE-TRANSFORMER - Conversor de Formatos")
    print("=" * 50)

    if len(sys.argv) < 2:
        print(" Uso: python The-Transformer.py <archivo> [--to json|csv|yaml]")
        print(" Conversiones automáticas:")
        print("   • CSV -> JSON")
        print("   • JSON -> CSV")
        print("   • JSON -> YAML")
        print("   • YAML -> JSON")
        print(" Ejemplo: python The-Transformer.py datos.csv")
        print("          python The-Transformer.py config.json --to yaml")
        return 1

    input_path = sys.argv[1]
    target_format = None

    if "--to" in sys.argv:
        idx = sys.argv.index("--to")
        if idx + 1 < len(sys.argv):
            target_format = sys.argv[idx + 1]

    if not Path(input_path).exists():
        print(f"❌ Archivo no encontrado: {input_path}")
        return 1

    input_ext = Path(input_path).suffix.lower()

    print(f"📥 Archivo de entrada: {input_path}")
    print(f"📤 Formato detectado: {input_ext}")

    if target_format:
        print(f"🎯 Formato objetivo: {target_format}")

    try:
        if input_ext == ".csv":
            output_path, data = csv_to_json(input_path)
            print(f"\n✅ Convertido: CSV -> JSON")
            print(f"   📄 Salida: {output_path}")
            print(f"   📊 Registros: {len(data)}")

        elif input_ext == ".json":
            if target_format == "csv":
                output_path, count = json_to_csv(input_path)
                print(f"\n✅ Convertido: JSON -> CSV")
                print(f"   📄 Salida: {output_path}")
                print(f"   📊 Registros: {count}")
            elif target_format in ("yaml", "yml"):
                output_path, data = json_to_yaml(input_path)
                print(f"\n✅ Convertido: JSON -> YAML")
                print(f"   📄 Salida: {output_path}")
            else:
                print("❌ Especifica --to csv o --to yaml")
                return 1

        elif input_ext in (".yaml", ".yml"):
            output_path, data = yaml_to_json(input_path)
            print(f"\n✅ Convertido: YAML -> JSON")
            print(f"   📄 Salida: {output_path}")
            print(f"   📊 Registros: {len(data) if isinstance(data, list) else 1}")

        else:
            print(f"❌ Formato no soportado: {input_ext}")
            return 1

    except Exception as e:
        print(f"❌ Error: {e}")
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
