#!/usr/bin/env python3
"""
POLYGOT - Traductor de Documentación
Traduce archivos .md a múltiples idiomas usando APIs.
Trigger: Ejecución manual cuando necesitas文档的多语言版本
"""

import os
import sys
import json
import http.client
from pathlib import Path

DEFAULT_LANGUAGES = ["en", "es", "pt"]


def translate_text(text, target_lang, api_key=None):
    if api_key is None:
        api_key = os.environ.get("DEEPL_API_KEY", "")

    if not api_key:
        print("⚠️ Sin API key de DeepL. Usando traducción básica.")
        return basic_translate(text, target_lang)

    try:
        conn = http.client.HTTPSConnection("api-free.deepl.com")
        headers = {
            "Authorization": f"DeepL-Auth-Key {api_key}",
            "Content-Type": "application/json",
        }

        payload = json.dumps({"text": [text], "target_lang": target_lang.upper()})

        conn.request("POST", "/v2/translate", payload, headers)
        response = conn.getresponse()
        data = json.loads(response.read())

        return data["translations"][0]["text"]

    except Exception as e:
        print(f"❌ Error en traducción: {e}")
        return text


def basic_translate(text, target_lang):
    translations = {
        "en": {"hola": "hello", "mundo": "world", "proyecto": "project"},
        "es": {"hello": "hola", "world": "mundo", "project": "proyecto"},
        "pt": {"hello": "ola", "world": "mundo", "project": "projeto"},
    }

    lang_dict = translations.get(target_lang, {})
    result = text.lower()
    for eng, trans in lang_dict.items():
        result = result.replace(eng, trans)
    return result


def translate_file(input_path, target_langs, api_key=None):
    with open(input_path, "r", encoding="utf-8") as f:
        content = f.read()

    input_ext = Path(input_path).suffix
    base_name = input_path.replace(input_ext, "")

    for lang in target_langs:
        print(f"🔄 Traduciendo a {lang}...")

        translated = translate_text(content, lang, api_key)

        output_path = f"{base_name}.{lang}{input_ext}"

        with open(output_path, "w", encoding="utf-8") as f:
            f.write(translated)

        print(f"   ✅ Guardado: {output_path}")


def main():
    print("🌐 POLYGOT - Traductor de Documentación")
    print("=" * 50)

    if len(sys.argv) < 2:
        print(" Uso: python Polyglot.py <archivo.md> [idiomas...]")
        print(" Idiomas: en, es, pt, fr, de, it, ja, zh")
        print(" Ejemplo: python Polyglot.py README.md en es")
        print("          python Polyglot.py docs.md")
        return 1

    input_path = sys.argv[1]

    if not Path(input_path).exists():
        print(f"❌ Archivo no encontrado: {input_path}")
        return 1

    target_langs = sys.argv[2:] if len(sys.argv) > 2 else DEFAULT_LANGUAGES

    print(f"📄 Archivo: {input_path}")
    print(f"🌍 Idiomas objetivo: {', '.join(target_langs)}\n")

    api_key = os.environ.get("DEEPL_API_KEY")
    if api_key:
        print("🔑 API Key de DeepL detectada\n")

    translate_file(input_path, target_langs, api_key)

    print("\n✅ Traducciones completadas!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
