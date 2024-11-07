use anyhow::Result;
use handlebars::Handlebars;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_json::json;

use cfg::EmailCfg;

use crate::template::register_templates;

pub enum EmailProvider {
    Aws,
}

pub struct Email<'a> {
    pub provider: EmailProvider,
    pub tmpl: Handlebars<'a>,
    pub smtp_server: String,
    pub username: String,
    pub password: String,
    pub from: String,
}

impl<'a> Email<'a> {
    pub fn new(provider: EmailProvider, cfg: &EmailCfg) -> Result<Self> {
        let tmpl = register_templates(&cfg.templates_path)?;

        match provider {
            EmailProvider::Aws => Ok(Self {
                provider,
                tmpl,
                smtp_server: cfg.aws.smtp_server.clone(),
                username: cfg.aws.username.clone(),
                password: cfg.aws.password.clone(),
                from: cfg.aws.from.clone(),
            }),
        }
    }

    pub fn send(&self, to: &str, name: &str, subject: &str, body: String) -> Result<()> {
        let creds = Credentials::new(self.username.clone(), self.password.clone());

        let mailer = SmtpTransport::relay(&self.smtp_server)?;

        let mailer = mailer.credentials(creds).build();

        let from = format!("Tawuno <{}>", self.from).parse()?;
        let to = format!("{} <{}>", name, to).parse()?;

        let message = Message::builder()
            .from(from)
            .to(to)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        mailer.send(&message)?;

        Ok(())
    }

    /// Send email verification link
    pub fn email_verification(&self, to: &str, to_name: &str, token: String) -> Result<()> {
        let body = self
            .tmpl
            .render("email_verification", &json!({"token": token}))?;

        self.send(to, to_name, "Email Verification Link", body)?;

        Ok(())
    }
}
