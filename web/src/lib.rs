use application::command::user::login_user::LoginUserCommand;
use askama::Template;
use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub name: &'a str,
}

pub async fn index() -> Html<String> {
    let template = IndexTemplate { name: "World" };

    Html(template.render().unwrap())
}

pub async fn hello() -> Html<String> {
    Html("<p>Hello from the server!</p>".to_string())
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/hello", get(hello))
}
