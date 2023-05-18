use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type Conn = Pool<Postgres>;

pub async fn connect(url: &str) -> Conn {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url).await.expect("Failed to connect to database");
    println!("Successfully connected to database");
    let row: (chrono::DateTime<chrono::Local>, String) = sqlx::query_as("SELECT now(), version()")
        .fetch_one(&pool).await.expect("Failed to test database connection");
    println!("Database time is {} and version {}", row.0, row.1);
    pool
}

pub async fn run_migrations(conn: &Conn) {
    println!("Running database migrations");
    sqlx::migrate!().run(conn).await.expect("Database migration failed")
}
