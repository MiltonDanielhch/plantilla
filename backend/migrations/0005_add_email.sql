-- Agregar columna email (nullable inicialmente para no romper usuarios existentes)
ALTER TABLE users ADD COLUMN email TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email ON users(email);