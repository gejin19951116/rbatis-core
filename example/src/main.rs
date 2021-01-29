use rbatis_core::db::DBPool;

fn main() -> Result<(), rbatis_core::Error> {
    async_std::task::block_on(async move {
        //Automatic judgment of database type
        let pool = DBPool::new("mysql://root:123456@localhost:3306/test").await?;
        let mut conn = pool.acquire().await?;
        let count: serde_json::Value = conn
            .fetch("SELECT count(1) FROM biz_activity;")
            .await.unwrap().0;
        println!("count: {}", count);
        return Ok(());
    })
}
