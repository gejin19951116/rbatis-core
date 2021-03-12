use rbatis_core::db::DBPool;
use rbatis_core::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //Automatic judgment of database type
    let pool = DBPool::new("mysql://root:123456@localhost:3306/test").await?;
    let mut conn = pool.acquire().await?;
    let count: serde_json::Value = conn
        .fetch("SELECT count(1) FROM biz_activity;")
        .await.unwrap().0;
    println!("count: {}", count);
    return Ok(());
}
