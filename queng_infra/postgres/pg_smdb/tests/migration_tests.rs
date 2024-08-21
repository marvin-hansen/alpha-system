use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use pg_smdb::run_smdb_db_migration;

fn postgres_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = String::from("postgres://postgres:postgres@localhost/postgres");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

fn test_db_migration(conn: &mut pg_smdb::Connection) {
    let res = run_smdb_db_migration(conn);
    //dbg!(&result);
    assert!(res.is_err());
}

#[test]
fn test_run_smdb_db_migration() {
    let pool = postgres_connection_pool();
    let conn = &mut pool.get().unwrap();

    println!("Test DB migration");
    test_db_migration(conn);
}
