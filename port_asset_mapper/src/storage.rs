use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

pub type DbPool = Pool;

pub async fn init_pool() -> Result<DbPool, Box<dyn std::error::Error>> {
    let mut cfg = Config::new();
    cfg.dbname = Some("asset_db".to_string());
    cfg.user = Some("test_user".to_string());
    cfg.password = Some("test_password".to_string());
    cfg.host = Some("localhost".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let pool = cfg.create_pool(None, NoTls)?;
    Ok(pool)
}
