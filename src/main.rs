use application::command::user::login_user::{LoginUserCommand, LoginUserCommandHandler};
use application::command::CommandHandler;
use application::shared::error::AppStatus;
use domain::repositories::id_provider::SimpleIdProvider;
use domain::repositories::otp_repository::InMemoryOtpRepository;
use domain::repositories::session_repository::InMemorySessionRepository;
use domain::repositories::user_repository::InMemoryUserRepository;
use domain::services::mail_service::InMemoryMailService;
use sqlx::PgPool;
use tokio::io;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

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
    Ok(input.trim().to_string()) // Trim to remove trailing newline
}


#[tokio::main]
async fn main() -> Result<(), ()> {
    // let pool = setup_database().await.expect("Failed to initialize database pool");
    //
    // if let Err(err) = test_db(&pool).await {
    //     eprintln!("Error running test_db: {}", err);
    // }

    let pool = persistence::init_db().await.expect("Failed to initialize database");

    let command = LoginUserCommand::new("test".to_owned(), None);

    let ur = InMemoryUserRepository::new();
    let sr = InMemorySessionRepository::new();
    let or = InMemoryOtpRepository::new();
    let idp = SimpleIdProvider::new();
    let ms = InMemoryMailService::new();

    let mut handler = LoginUserCommandHandler::new(ur, sr, or, idp, ms);

    match handler.handle(command).await {
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

    match handler.handle(LoginUserCommand::new("test".to_owned(), Some(input))).await {
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

    // let handler = LoginUserCommandHandler::new();
    //
    // match handler.handle(command) {
    //     Ok(msg) => println!("Success: {}", msg),
    //     Err(err) => eprintln!("Error: {}", err),
    // }


    Ok(())
}
