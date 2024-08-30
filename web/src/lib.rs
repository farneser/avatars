use application::AppContainer;
use askama::Template;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tokio::net;
use tower_http::services::ServeDir;

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

fn get_router(container: Arc<AppContainer>) -> Router {
    let static_files_router = Router::new()
        .fallback_service(ServeDir::new("./web/static"));

    let app_routes = Router::new()
        .route("/", get(index))
        .route("/hello", get(hello));

    Router::new()
        .nest_service("/static", static_files_router)
        .merge(app_routes)
        .with_state(container)
}

pub struct Server
{
    port: u16,
    container: Arc<AppContainer>,
}

impl Server
{
    pub fn new(port: u16, container: Arc<AppContainer>) -> Self {
        Server { port, container }
    }

    pub async fn run(self) {
        let router = get_router(self.container);

        let addr = format!("0.0.0.0:{}", self.port);

        let listener = net::TcpListener::bind(&addr).await.unwrap();

        axum::serve(listener, router).await.unwrap();
    }
}