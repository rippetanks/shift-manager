
extern crate postgres;

//use postgres::Connection;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::{Pool, PooledConnection};
//use postgres::tls::native_tls::NativeTls;
//use postgres::tls::openssl::OpenSsl;

static mut POOL: Option<Pool<PostgresConnectionManager>> = None;

pub fn init() {
    let manager = PostgresConnectionManager::new(
        "postgres://tm:tm_postgres@localhost:5432/turnimanager",
        TlsMode::None).unwrap();
    let pool = r2d2::Pool::builder().max_size(2).build(manager);
    match pool {
        Ok(pool) => {
            debug!("Pool Postgres OK!");
            unsafe {
                POOL = Some(pool);
            }
        },
        Err(e) => {
            error!("DB Pool failed!");
            error!("{}", e);
        }
    }
}

pub fn get_connection() -> PooledConnection<PostgresConnectionManager> {
    unsafe {
        POOL.as_ref().expect("").get().unwrap()
    }
}
/*
pub fn establish_connection() -> Connection {
    //let negotiator = OpenSsl::new().unwrap(); // TlsMode::Require(&negotiator)
    Connection::connect("postgres://pg:postgres@rippetanks.ddns.net:5432/turnimanager",
                        postgres::TlsMode::None)
        .unwrap()
}
*/