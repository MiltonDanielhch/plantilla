pub mod audit;
pub mod auth;
pub mod common;
pub mod dashboard;
pub mod roles;
pub mod users;

// Re-exports para compatibilidad hacia atrás
pub use audit::{export_audit_logs, export_users, get_audit_logs, get_stats};
pub use auth::{
    change_password, forgot_password, login, logout, logout_all, refresh_token, reset_password,
    send_verification_email, verify_email,
};
pub use dashboard::dashboard;
pub use roles::{
    create_role, delete_role, get_permissions, get_role_permissions, get_roles, update_permission,
    update_role,
};
pub use users::{
    create_user, delete_user, get_user_by_id, get_users, update_user, upload_avatar,
};
