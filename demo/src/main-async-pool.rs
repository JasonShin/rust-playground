use anyhow;
use tokio;
use tokio::runtime::Runtime;
use sqlx::{mysql::{MySqlPool, MySqlPoolOptions}, Row};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rt = Runtime::new().unwrap();
    let local = tokio::task::LocalSet::new();
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://root@localhost:33306/sqlx-ts")
        .await?;

    let result = sqlx::query("SELECT id, food_type FROM items")
        .fetch_all(&pool)
        .await?;

    // https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("food_type"));
    }
   
    println!("testing");
    Ok(())
}
