use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

async fn setup_database() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://avatars:avatars@localhost/avatars")
        .await?;

    Ok(pool)
}

async fn run_migrations(pool: &PgPool) -> Result<(), String> {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let migrations_path = current_dir.join("migrations");

    let migrator = sqlx::migrate::Migrator::new(migrations_path.as_path())
        .await
        .map_err(|e| format!("Failed to initialize migrator: {}", e))?;

    migrator.run(pool).await.map_err(|e| format!("Failed to run migrations: {}", e))?;

    Ok(())
}


pub async fn init_db() -> Result<PgPool, String> {
    let pool_result = setup_database().await;

    if let Err(err) = pool_result {
        eprintln!("Error setting up database: {}", err);

        return Err(err.to_string());
    }

    let pool = pool_result.unwrap();

    if let Err(err) = run_migrations(&pool).await {
        eprintln!("Error running migrations: {}", err);

        return Err(err);
    }

    Ok(pool)
}