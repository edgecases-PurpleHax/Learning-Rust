use crate::storage::DbPool;

pub async fn run_migration(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let sql = std::fs::read_to_string("migrations/001_init.sql")?;
    client.batch_execute(&sql).await?;
    Ok(())
}
