use rbatis_core::db::DBPool;
use rbatis_core::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //Automatic judgment of database type
    let pool = DBPool::new("mysql://root:123456@localhost:3306/test").await?;
    let mut conn = pool.acquire().await?;
    let data:(serde_json::Value,usize) = conn
        .fetch("SELECT * FROM biz_activity;")
        .await.unwrap();
    println!("count: {:?}", data);
    return Ok(());
}
