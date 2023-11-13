use std::time::Duration;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub type Db = Pool<Postgres>;

async fn new_db_pool(host: &str,db: &str, user: &str, pwd: &str, max_con: u32) -> Result<Db, sqlx::Error> {
  let con_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
  
  PgPoolOptions::new()
  .max_connections(max_con)
  .idle_timeout(Duration::from_millis(500))
  .connect(&con_string)
  .await

}
