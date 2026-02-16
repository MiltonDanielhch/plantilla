use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message,
};
use tracing;

pub struct EmailService {
    mailer: AsyncSmtpTransport<lettre::Tokio1Executor>,
    from_email: String,
}

impl EmailService {
    pub fn new(smtp_host: &str, smtp_port: u16, smtp_user: &str, smtp_pass: &str, from_email: &str) -> Self {
        let creds = Credentials::new(smtp_user.to_string(), smtp_pass.to_string());
        
        let mailer = AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(smtp_host)
            .unwrap()
            .port(smtp_port)
            .credentials(creds)
            .build();
        
        Self {
            mailer,
            from_email: from_email.to_string(),
        }
    }
    
    pub async fn send_password_reset(&self, to_email: &str, reset_token: &str, username: &str) -> Result<(), String> {
        let reset_url = format!("http://localhost:4321/reset-password?token={}", reset_token);
        
        let subject = "Recuperación de Contraseña - Sintonía 3026";
        
        let body = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9f9f9; padding: 30px; border-radius: 0 0 8px 8px; }}
        .button {{ display: inline-block; background: #667eea; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; margin: 20px 0; }}
        .footer {{ text-align: center; color: #666; margin-top: 20px; font-size: 12px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Sintonía 3026</h1>
            <p>Recuperación de Contraseña</p>
        </div>
        <div class="content">
            <h2>Hola {},</h2>
            <p>Has solicitado restablecer tu contraseña. Haz clic en el siguiente enlace para crear una nueva contraseña:</p>
            <center>
                <a href="{}" class="button">Restablecer Contraseña</a>
            </center>
            <p>O copia y pega este enlace en tu navegador:</p>
            <p style="background: #eee; padding: 10px; border-radius: 4px; word-break: break-all;">{}</p>
            <p><strong>Este enlace expirará en 1 hora.</strong></p>
            <p>Si no solicitaste restablecer tu contraseña, puedes ignorar este correo.</p>
        </div>
        <div class="footer">
            <p>Este es un correo automático de Sintonía 3026. No respondas a este mensaje.</p>
        </div>
    </div>
</body>
</html>
            "#,
            username, reset_url, reset_url
        );
        
        let email = Message::builder()
            .from(self.from_email.parse().map_err(|e| format!("Invalid from email: {}", e))?)
            .to(to_email.parse().map_err(|e| format!("Invalid to email: {}", e))?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body)
            .map_err(|e| format!("Error building email: {}", e))?;
        
        match self.mailer.send(email).await {
            Ok(_) => {
                tracing::info!("Password reset email sent to {}", to_email);
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to send email to {}: {}", to_email, e);
                Err(format!("Failed to send email: {}", e))
            }
        }
    }
}

// Factory function para crear el servicio desde variables de entorno
pub fn create_email_service() -> Option<EmailService> {
    let smtp_host = std::env::var("SMTP_HOST").ok()?;
    let smtp_port = std::env::var("SMTP_PORT").ok()?.parse::<u16>().ok()?;
    let smtp_user = std::env::var("SMTP_USER").ok()?;
    let smtp_pass = std::env::var("SMTP_PASS").ok()?;
    let from_email = std::env::var("FROM_EMAIL").ok()?;
    
    Some(EmailService::new(&smtp_host, smtp_port, &smtp_user, &smtp_pass, &from_email))
}