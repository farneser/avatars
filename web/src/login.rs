use application::command::user::login_user::LoginUserCommand;
use application::shared::error::AppStatus;
use application::AppContainer;
use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::Form;
use serde::Deserialize;
use std::sync::Arc;
use axum::http::HeaderMap;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<> {}

pub async fn login_get() -> Html<String> {
    let template = LoginTemplate {};

    Html(template.render().unwrap())
}

#[derive(Deserialize)]
pub struct EmailData {
    email: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    email: String,
    otp: Option<String>,
}

pub(crate) async fn handle_email(
    State(container): State<Arc<AppContainer>>,
    Form(data): Form<EmailData>,
) -> Html<String> {
    println!("Received email: {}", data.email.clone());

    match container.send_command(LoginUserCommand::new(data.email.clone(), None)).await {
        Ok(_) => {}
        Err(err) => {
            match err {
                AppStatus::Ok(_) => {
                    println!("Email sent successfully.")
                }
                _ => { return Html("Error sending email.".to_owned()) }
            }
        }
    };

    let response = format!(
        r#"
        <h2>Enter OTP</h2>
        <form id="otp-form" hx-post="/login" hx-swap="innerHTML">
            <input type="hidden" name="email" value="{email}">
            <label for="otp">OTP:</label>
            <input type="text" id="otp" name="otp" required>
            <button type="submit">Submit</button>
        </form>
        "#,
        email = data.email
    );

    // Return Html with the response String
    Html(response)
}

pub(crate) async fn handle_login(
    State(container): State<Arc<AppContainer>>,
    Form(data): Form<LoginData>,
) -> impl IntoResponse {

    let mut headers = HeaderMap::new();

    match container.send_command(LoginUserCommand::new(data.email.clone(), data.otp.clone())).await {
        Ok(s) => {
            headers.insert("Set-Cookie", format!("sessionId={}; Path=/; HttpOnly", s.value).parse().unwrap());

            (headers, "Login successful.".to_owned())
        }
        Err(_) => {
            (headers, "Login failed.".to_owned())
        }
    }
}
