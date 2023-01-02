use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

pub type ConnPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> ConnPool {
    dotenv().ok();

    let db_url = env::var("DB_URL").expect("no DB URL");
    let migr = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(migr)
        .expect("could not build connection pool")
}