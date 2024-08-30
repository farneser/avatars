use application::command::user::login_user::LoginUserCommand;
use application::shared::error::AppStatus;
use application::AppContainer;
use axum::Router;
use domain::repositories::id_provider::SimpleIdProvider;
use domain::repositories::otp_repository::InMemoryOtpRepository;
use domain::repositories::session_repository::InMemorySessionRepository;
use domain::repositories::user_repository::InMemoryUserRepository;
use domain::services::mail_service::InMemoryMailService;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::io;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tower_http::services::ServeDir;
use web::create_router;

async fn test_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    for i in 10..100 {
        let name = format!("Alice{i}");
        sqlx::query(r#"
            INSERT INTO persons (name, age)
            VALUES ($1, $2)
        "#)
            .bind(&name)
            .bind(30)
            .execute(pool)
            .await?;

        println!("Inserted person with name: {}", name);
    }

    let people = sqlx::query_as::<_, (String, i64)>(r#"
        SELECT name, age
        FROM persons
    "#)
        .fetch_all(pool)
        .await?;

    for (name, age) in people {
        println!("Name: {}, Age: {}", name, age);
    }

    Ok(())
}

async fn read_input() -> io::Result<String> {
    let mut input = String::new();
    let stdin = stdin();
    let mut reader = BufReader::new(stdin);

    reader.read_line(&mut input).await?;
    Ok(input.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    // let pool = setup_database().await.expect("Failed to initialize database pool");
    //
    // if let Err(err) = test_db(&pool).await {
    //     eprintln!("Error running test_db: {}", err);
    // }

    let static_files_router = Router::new()
        .fallback_service(ServeDir::new("./web/static"));

    // Основной маршрутизатор
    let app = Router::new()
        .merge(create_router())
        .nest_service("/static", static_files_router);


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on {}", addr);

    // let listener = TcpListener::bind(&addr).await
    //     .expect("Failed to bind to address");
    //
    // if let Err(e) = axum::serve(listener, app).await {
    //     eprintln!("Server error: {}", e);
    // }

    let pool = persistence::init_db().await.expect("Failed to initialize database");

    let command = LoginUserCommand::new("test".to_owned(), None);

    let container = AppContainer::new(
        InMemoryUserRepository::new(),
        InMemorySessionRepository::new(),
        InMemoryOtpRepository::new(),
        SimpleIdProvider::new(),
        InMemoryMailService::new(),
    );

    match container.send_command(command).await {
        Ok(u) => println!("User: {:?}", u),
        Err(err) => {
            match err {
                AppStatus::Ok(msg) => {
                    println!("Success: {}", msg);
                }
                _ => {
                    eprintln!("Error: {}", err);
                }
            }
        }
    }

    let input = read_input().await.unwrap();

    println!("You entered: {}", input.clone());

    match container.send_command(LoginUserCommand::new("test".to_owned(), Some(input))).await {
        Ok(u) => println!("User: {:?}", u),
        Err(err) => {
            match err {
                AppStatus::Ok(msg) => {
                    println!("Success: {}", msg);
                }
                _ => {
                    eprintln!("Error: {}", err);
                }
            }
        }
    }

    Ok(())
}