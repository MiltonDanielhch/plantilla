-- Tabla de Auditoría para registrar acciones administrativas
CREATE TABLE audit_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    admin_username TEXT NOT NULL, -- Quién ejecutó la acción
    action TEXT NOT NULL,         -- Qué hizo (ej: "DELETE_USER")
    target TEXT NOT NULL,         -- A quién afectó (ej: "usuario_borrado")
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);