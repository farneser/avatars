pub mod adapters;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::path::Path;

async fn setup_database() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://avatars:avatars@localhost/avatars")
        .await?;

    Ok(pool)
}

async fn run_migrations(pool: &PgPool, migrations_path: String) -> Result<(), String> {
    let migrator = sqlx::migrate::Migrator::new(Path::new(migrations_path.as_str()))
        .await
        .map_err(|e| format!("Failed to initialize migrator: {}", e))?;

    migrator.run(pool).await.map_err(|e| format!("Failed to run migrations: {}", e))?;

    Ok(())
}
mod dev_demo {
    use domain::models::user::User;
    use sqlx::PgPool;

    async fn insert_user(pool: &PgPool, email: &str) -> Result<(), String> {
        let user = User::new(email.to_owned());

        sqlx::query(
            r#"
        INSERT INTO users (username, register_date, last_update_date)
        VALUES ($1, $2, $3)
        "#,
        )
            .bind(&user.username)
            .bind(user.register_date)
            .bind(user.last_update_date)
            .execute(pool)
            .await.expect("Failed to insert user");

        Ok(())
    }

    async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Option<User> {
        let query = "SELECT id, username, register_date, last_update_date FROM users WHERE id = $1";

        let user = sqlx::query_as::<_, User>(query)
            .bind(user_id)
            .fetch_optional(pool)
            .await.expect("Failed to fetch user");

        user
    }
}

pub async fn init_db() -> Result<PgPool, String> {
    let pool_result = setup_database().await;

    if let Err(err) = pool_result {
        eprintln!("Error setting up database: {}", err);

        return Err(err.to_string());
    }

    let pool = pool_result.unwrap();

    if let Err(err) = run_migrations(&pool, "persistence/migrations".to_string()).await {
        eprintln!("Error running migrations: {}", err);

        return Err(err);
    }

    Ok(pool)
}