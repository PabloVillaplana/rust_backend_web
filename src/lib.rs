#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    return MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
}

// Esta funcion se encarga de establecer la conexion con la base de datos y devolver un pool de conexiones (Un pool de conexiones es un conjunto de conexiones que se pueden reutilizar)
pub fn establish_pool_connection() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    return Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
}
