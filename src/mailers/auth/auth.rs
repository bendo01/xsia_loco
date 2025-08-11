// auth mailer
#![allow(non_upper_case_globals)]

use crate::common::settings::Settings;
use crate::models::auth::users::users;
use loco_rs::prelude::*;
use serde_json::json;

static welcome: Dir<'_> = include_dir!("src/mailers/auth/welcome");
static forgot: Dir<'_> = include_dir!("src/mailers/auth/forgot");
static magic_link: Dir<'_> = include_dir!("src/mailers/auth/magic_link");
// #[derive(Mailer)] // -- disabled for faster build speed. it works. but lets
// move on for now.
//
pub const DEFAULT_FROM_SENDER: &str = "No Reply <no-reply@xsia.app>";

#[allow(clippy::module_name_repetitions)]
pub struct AuthMailer {}
impl Mailer for AuthMailer {}
impl AuthMailer {
    /// Sending welcome email the the given user
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_welcome(ctx: &AppContext, user: &users::Model) -> Result<()> {
        if let Some(settings) = &ctx.config.settings {
            let settings = Settings::from_json(settings)?;
            let institution_code = settings.current_institution_code;
            let server_domain = settings.server_domain;
            Self::mail_template(
                ctx,
                &welcome,
                mailer::Args {
                    from: Some(DEFAULT_FROM_SENDER.to_string()),
                    to: user.email.to_string(),
                    locals: json!({
                      "name": user.name,
                      "verifyToken": user.email_verification_token,
                      "domain": server_domain,
                      "institution_code": institution_code
                    }),
                    ..Default::default()
                },
            )
            .await?;
        }
        Ok(())
    }

    /// Sending forgot password email
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn forgot_password(ctx: &AppContext, user: &users::Model) -> Result<()> {
        if let Some(settings) = &ctx.config.settings {
            let settings = Settings::from_json(settings)?;
            let institution_code = settings.current_institution_code;
            let server_domain = settings.server_domain;
            Self::mail_template(
                ctx,
                &forgot,
                mailer::Args {
                    from: Some(DEFAULT_FROM_SENDER.to_string()),
                    to: user.email.to_string(),
                    locals: json!({
                      "name": user.name,
                      "resetToken": user.reset_token,
                      "domain": server_domain,
                      "institution_code": institution_code
                    }),
                    ..Default::default()
                },
            )
            .await?;
        }
        Ok(())
    }

    /// Sends a magic link authentication email to the user.
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_magic_link(ctx: &AppContext, user: &users::Model) -> Result<()> {
        Self::mail_template(
            ctx,
            &magic_link,
            mailer::Args {
                from: Some(DEFAULT_FROM_SENDER.to_string()),
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "token": user.magic_link_token.clone().ok_or_else(|| Error::string(
                            "the user model not contains magic link token",
                    ))?,
                  "host": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }
}
