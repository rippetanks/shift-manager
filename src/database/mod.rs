
use postgres::rows::Rows;
use postgres::Error;

pub mod user_db;
pub mod shift_structure_db;
pub mod shift_expansion_db;
pub mod db;

fn handle_pg_result(result: Result<Rows, Error>) -> Result<Rows, ()> {
    match result {
        Ok(rows) => Ok(rows),
        Err(e) => {
            error!("Postgres Result Error!");
            error!("{}", e);
            Err(())
        }
    }
}

fn handle_pg_update(result: Result<u64, Error>) -> Result<u64, ()> {
    match result {
        Ok(n) => {
            debug!("Postgres Update n={}", n);
            Ok(n)
        },
        Err(e) => {
            error!("Postgres Update Error!");
            error!("{}", e);
            Err(())
        }
    }
}