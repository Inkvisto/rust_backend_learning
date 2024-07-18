use uuid::Uuid;
use sqlx::postgres::PgPoolOptions;

#[tokio::main] 
async fn main() -> Result<(), sqlx::Error> {
    let uuid = Uuid::new_v4();
    let uuid_v7 = Uuid::now_v7();

    dbg!(uuid);
    dbg!(uuid_v7);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://egor:root@localhost/test").await?;

    let row: (i64,) = sqlx::query_as("SELECT * from keys")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    dbg!(row);


    Ok(())
}
