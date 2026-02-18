import sqlite3

try:
    conn = sqlite3.connect('backend.db')
    cursor = conn.cursor()
    cursor.execute("SELECT id, username, role FROM users")
    users = cursor.fetchall()
    print(f"Total usuarios encontrados: {len(users)}")
    for user in users:
        print(f"ID: {user[0]}, User: {user[1]}, Role: {user[2]}")
    conn.close()
except Exception as e:
    print(f"Error: {e}")
