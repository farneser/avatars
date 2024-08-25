// use sqlx::PgPool;
// use sqlx::postgres::PgPoolOptions;
use application::command::CommandHandler;

use application::command::user::register_user::{RegisterUserCommand, RegisterUserCommandHandler};

// async fn test_db(pool: &PgPool) -> Result<(), sqlx::Error> {
//     for i in 10..100 {
//         let name = format!("Alice{i}");
//         sqlx::query(r#"
//             INSERT INTO persons (name, age)
//             VALUES ($1, $2)
//         "#)
//             .bind(&name)
//             .bind(30)
//             .execute(pool)
//             .await?;
//
//         println!("Inserted person with name: {}", name);
//     }
//
//     let people = sqlx::query_as::<_, (String, i64)>(r#"
//         SELECT name, age
//         FROM persons
//     "#)
//         .fetch_all(pool)
//         .await?;
//
//     for (name, age) in people {
//         println!("Name: {}, Age: {}", name, age);
//     }
//
//     Ok(())
// }


#[tokio::main]
async fn main() -> Result<(), ()> {
    // let pool = setup_database().await.expect("Failed to initialize database pool");
    //
    // if let Err(err) = test_db(&pool).await {
    //     eprintln!("Error running test_db: {}", err);
    // }

    persistence::init_db().await.expect("Failed to initialize database");

    let command = RegisterUserCommand::new("фррфрфрфрф".to_owned(), "фрфррфрф".to_owned());
    let handler = RegisterUserCommandHandler::new();

    match handler.handle(command) {
        Ok(msg) => println!("Success: {}", msg),
        Err(err) => eprintln!("Error: {}", err),
    }


    Ok(())
}
