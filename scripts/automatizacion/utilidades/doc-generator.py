#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
DOC-GENERATOR - Generador de Documentación
Lee comentarios y genera página HTML con la documentación técnica.
Trigger: Post-merge hook o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import re
import json
from pathlib import Path
from datetime import datetime

OUTPUT_DIR = Path("docs/generated")


def extract_docstrings(filepath):
    docstrings = []

    try:
        with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
            content = f.read()

        if filepath.endswith(".py"):
            pattern = r'def\s+(\w+)\s*\([^)]*\):\s*"""([^"]*)"""'
            matches = re.findall(pattern, content)
            for func_name, doc in matches:
                if doc.strip():
                    docstrings.append(
                        {
                            "type": "function",
                            "name": func_name,
                            "doc": doc.strip(),
                            "file": str(filepath),
                        }
                    )

            pattern = r'class\s+(\w+)\s*(?:\([^)]*\))?:\s*"""([^"]*)"""'
            matches = re.findall(pattern, content)
            for class_name, doc in matches:
                if doc.strip():
                    docstrings.append(
                        {
                            "type": "class",
                            "name": class_name,
                            "doc": doc.strip(),
                            "file": str(filepath),
                        }
                    )

        elif filepath.endswith(".rs"):
            pattern = r"///\s*(.+?)$|//!\s*(.+?)$"
            matches = re.findall(pattern, content, re.MULTILINE)
            if matches:
                docstrings.append(
                    {
                        "type": "doc_comment",
                        "name": filepath.stem,
                        "doc": " ".join([m[0] or m[1] for m in matches]).strip(),
                        "file": str(filepath),
                    }
                )

    except Exception as e:
        pass

    return docstrings


def scan_project(root_dir):
    all_docs = []

    for ext in ["*.py", "*.rs", "*.js", "*.ts"]:
        for filepath in Path(root_dir).rglob(ext):
            if any(
                x in str(filepath)
                for x in ["node_modules", "target", "__pycache__", ".venv"]
            ):
                continue

            docs = extract_docstrings(filepath)
            all_docs.extend(docs)

    return all_docs


def generate_html(docs):
    html = (
        """<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Documentación - Sintonía 3026</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 1200px; margin: 0 auto; padding: 20px; background: #1a1a2e; color: #eee; }
        h1 { color: #00d4ff; border-bottom: 2px solid #00d4ff; padding-bottom: 10px; }
        h2 { color: #ff6b6b; margin-top: 30px; }
        .doc-item { background: #16213e; padding: 15px; margin: 10px 0; border-radius: 8px; border-left: 4px solid #00d4ff; }
        .doc-item.class { border-left-color: #ff6b6b; }
        .doc-item.function { border-left-color: #4ecdc4; }
        .doc-name { font-weight: bold; font-size: 1.1em; }
        .doc-doc { margin-top: 8px; color: #bbb; }
        .doc-file { font-size: 0.8em; color: #666; margin-top: 5px; }
        .badge { display: inline-block; padding: 2px 8px; border-radius: 4px; font-size: 0.7em; margin-left: 10px; }
        .badge.class { background: #ff6b6b; }
        .badge.function { background: #4ecdc4; }
        .badge.doc_comment { background: #ffe66d; color: #333; }
        .search-box { width: 100%; padding: 12px; font-size: 16px; background: #0f3460; border: none; color: #fff; border-radius: 8px; margin-bottom: 20px; }
        .search-box::placeholder { color: #888; }
    </style>
</head>
<body>
    <h1>📚 Documentación - Sintonía 3026</h1>
    <p>Generado automáticamente el """
        + datetime.now().strftime("%Y-%m-%d %H:%M")
        + """</p>
    <input type="text" class="search-box" placeholder="Buscar funciones, clases..." onkeyup="filterDocs(this.value)">
    <div id="docs">
"""
    )

    for doc in docs:
        badge_class = doc["type"].replace("_", "-")
        html += f'''
        <div class="doc-item {doc["type"]}" data-search="{doc["name"].lower()} {doc["doc"].lower()}">
            <span class="doc-name">{doc["name"]}</span>
            <span class="badge {badge_class}">{doc["type"]}</span>
            <div class="doc-doc">{doc["doc"]}</div>
            <div class="doc-file">📄 {doc["file"]}</div>
        </div>
'''

    html += """
    </div>
    <script>
        function filterDocs(query) {
            query = query.toLowerCase();
            document.querySelectorAll('.doc-item').forEach(item => {
                item.style.display = item.dataset.search.includes(query) ? 'block' : 'none';
            });
        }
    </script>
</body>
</html>"""

    return html


def main():
    print("[DOC-GENERATOR] Generador de Documentación")
    print("=" * 50)

    root_dir = os.getcwd()
    print(f"[INFO] Escaneando: {root_dir}")

    docs = scan_project(root_dir)
    print(f"[INFO] Documentos encontrados: {len(docs)}")

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    output_file = OUTPUT_DIR / "api_docs.html"

    html = generate_html(docs)
    with open(output_file, "w", encoding="utf-8") as f:
        f.write(html)

    print(f"[OK] Documentación generada: {output_file}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
