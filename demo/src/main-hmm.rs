use anyhow;
use tokio;
use sqlx::{postgres::{PgPoolOptions}, Row};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgresql://postgres:postgres@127.0.0.1:54321/postgres")
        .await?;
 
    let result = sqlx::query("SELECT id, food_type FROM items")
        .fetch_all(&pool)
        .await?;

    // https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("food_type"));
    }

    for _ in 0..5 {
        let _ = sqlx::query("PREPARE sqlx_stmts AS SELECT id, food_type FROM items")
        .execute(&pool)
        .await?;
    }
    
    // let _ = local.block_on(&mut rt, sqlx::query("DEALLOCATE sqlx_stmts").execute(&pool)).unwrap();
   
    println!("testing");
    Ok(())
}
