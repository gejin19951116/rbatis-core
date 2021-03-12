use rbatis_core::db::DBPool;

#[tokio::main]
async fn main() {
    //Automatic judgment of database type
    let pool = DBPool::new("mysql://root:123456@localhost:3306/test").await.unwrap();
    let mut conn = pool.acquire().await.unwrap();
    let count: serde_json::Value = conn
        .fetch("SELECT count(1) FROM biz_activity;")
        .await.unwrap().0;
    println!("count: {}", count);
}
