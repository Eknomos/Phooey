use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime, PoolConfig, Pool};
// use bb8::Pool;
// use bb8_postgres::PostgresConnectionManager;
// use tokio_postgres::NoTls;


// TODOS:
// Add TLS
pub fn get_db_pool() -> Pool
{
    let mut config: Config = Config::new();
    config.pool = Some(PoolConfig::new(16));
    config.port = Some(5432);
    config.host = Some("127.0.0.1".to_string());
    config.dbname = Some("phooey".to_string());
    config.password = Some("password123".to_string());
    config.user = Some("master".to_string());
    config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast});
    
    let pool: Pool = config.create_pool(Some(Runtime::Tokio1),
            tokio_postgres::NoTls
        )
        .expect("Error creating connection pool");
    return pool;
}


// let manager = PostgresConnectionManager::new_from_stringlike(
//     "postgresql://master:password123@127.0.0.1:5432/phooey", NoTls).unwrap();
    
 // let pool = Pool::builder().max_size(16).build(manager).await.unwrap();