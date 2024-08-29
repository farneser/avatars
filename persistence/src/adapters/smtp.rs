use async_trait::async_trait;
use domain::models::otp::Otp;
use domain::services::mail_service::{EmailError, MailService};
use lettre::message::{header, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct SmtpService {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub email_from: Mailbox,
}

impl SmtpService {
    pub fn new(host: String, port: u16, username: String, password: String, email_from: String) -> Result<Self, String> {
        let from = email_from.parse::<Mailbox>();

        if let Err(_) = from {
            return Err("Invalid email address".to_owned());
        }

        Ok(Self {
            host,
            port,
            username,
            password,
            email_from: from.unwrap(),
        })
    }
}

#[async_trait]
impl MailService for SmtpService {
    async fn send(&self, email: &str, subject: &str, html_body: &str, plain_body: &str) -> Result<(), EmailError> {
        if email.parse::<Mailbox>().is_err() {
            return Err(EmailError::InvalidMail("Invalid email address".to_owned()));
        }

        let email = Message::builder()
            .from(self.email_from.clone())
            .to(email.parse::<Mailbox>().unwrap())
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(plain_body.to_string()), // Ensure body is a String
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html_body.to_string()), // Ensure body is a String
                    ),
            )
            .map_err(|e| EmailError::InvalidMail(e.to_string()))?; // Handle email building error

        let creds = Credentials::new(self.username.clone(), self.password.clone());

        let mailer = SmtpTransport::builder_dangerous(&self.host)
            .port(self.port)
            .credentials(creds)
            .build();

        mailer.send(&email).map_err(|e| EmailError::InvalidMail(e.to_string()))?; // Handle send error

        Ok(())
    }

    async fn send_otp(&self, email: &str, otp: Otp) -> Result<(), EmailError> {
        self.send(
            email,
            "Your OTP",
            &format!("Your OTP is: {}", otp.id),
            &format!("Your OTP is: {}", otp.id),
        ).await
    }
}
