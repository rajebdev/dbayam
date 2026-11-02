//! Registration of the database drivers compiled into this binary.

use dbayam_core::registry::DriverRegistry;
use std::sync::Arc;

/// Register every built-in driver.
pub fn register_all(reg: &mut DriverRegistry) {
    dbayam_driver_postgres::register(reg);
    dbayam_driver_mysql::register(reg);
    reg.register(Arc::new(dbayam_driver_mssql::SqlServerDriver::new()));
    reg.register(Arc::new(dbayam_driver_clickhouse::ClickHouseDriver::new()));
    reg.register(Arc::new(dbayam_driver_sqlite::SqliteDriver::new()));
    reg.register(Arc::new(dbayam_driver_mongodb::MongoDriver::new()));
    reg.register(Arc::new(dbayam_driver_redis::RedisDriver::new()));
    #[cfg(feature = "duckdb")]
    reg.register(Arc::new(dbayam_driver_duckdb::DuckDbDriver::new()));
}
