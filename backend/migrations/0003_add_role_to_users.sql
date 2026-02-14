--- /dev/null
-- Agregar columna de rol a la tabla users
-- Por defecto, todos los usuarios existentes y nuevos serán 'user'
ALTER TABLE users ADD COLUMN role TEXT NOT NULL DEFAULT 'user';
