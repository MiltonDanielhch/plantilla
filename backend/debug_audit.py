import sqlite3
import json

try:
    conn = sqlite3.connect('backend.db')
    # Configurar row_factory para obtener diccionarios
    conn.row_factory = sqlite3.Row
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM audit_logs LIMIT 5")
    rows = cursor.fetchall()
    
    print(f"Total logs encontrados: {len(rows)}")
    for row in rows:
        # Convertir a dict para imprimir
        print(dict(row))
        
    conn.close()
except Exception as e:
    print(f"Error: {e}")
