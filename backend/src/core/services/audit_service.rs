use crate::core::models::user::{AuditLog, User};
use crate::core::repository::Repository;
use crate::error::AppError;
use csv::WriterBuilder;

pub struct AuditService<R: Repository> {
    repository: R,
}

impl<R: Repository> AuditService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    // ============ AUDIT LOGS ============

    pub async fn get_audit_logs(&self) -> Result<Vec<AuditLog>, AppError> {
        self.repository.get_audit_logs().await
    }

    pub async fn export_audit_logs(&self) -> Result<Vec<u8>, AppError> {
        let logs = self.repository.get_audit_logs().await?;
        self.build_audit_csv(logs)
    }

    fn build_audit_csv(&self, logs: Vec<AuditLog>) -> Result<Vec<u8>, AppError> {
        let mut wtr = WriterBuilder::new().has_headers(true).from_writer(vec![]);

        wtr.write_record(&["ID", "Admin Username", "Action", "Target", "Timestamp"])
            .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))))?;

        for log in logs {
            wtr.write_record(&[
                log.id.to_string(),
                log.admin_username,
                log.action,
                log.target,
                log.timestamp,
            ])
            .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))))?;
        }

        wtr.into_inner().map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))))
    }

    // ============ USER EXPORT ============

    pub async fn export_users(&self) -> Result<Vec<u8>, AppError> {
        let (users, _) = self.repository.get_all(None, 1, 10000).await?;
        self.build_users_csv(users)
    }

    fn build_users_csv(&self, users: Vec<User>) -> Result<Vec<u8>, AppError> {
        let mut wtr = WriterBuilder::new().has_headers(true).from_writer(vec![]);

        wtr.write_record(&["ID", "Username", "Email", "Role", "Created At"])
            .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))))?;

        for user in users {
            wtr.write_record(&[
                user.id.to_string(),
                user.username,
                user.email.unwrap_or_default(),
                format!("{:?}", user.role),
                user.created_at,
            ])
            .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))))?;
        }

        wtr.into_inner().map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))))
    }

    // ============ STATS ============

    pub async fn get_stats(&self) -> Result<Stats, AppError> {
        let (total, admins, new_today) = self.repository.get_stats().await?;
        
        Ok(Stats {
            total_users: total,
            active_users: total,
            admin_users: admins,
            new_users_today: new_today,
        })
    }
}

pub struct Stats {
    pub total_users: i64,
    pub active_users: i64,
    pub admin_users: i64,
    pub new_users_today: i64,
}
