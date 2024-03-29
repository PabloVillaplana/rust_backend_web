#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema; // Modelos

use actix_web::post;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use platzi_rust_blog_post::establish_pool_connection;

use self::models::Post;
use self::schema::posts::dsl::*;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    match pool.get() {
        Ok(conn) => {
            match web::block(move || {
                let mut conn = conn;
                posts.load::<Post>(&mut conn)
            })
            .await
            {
                Ok(data) => {
                    let mut respuesta = String::new();

                    for post in data.unwrap() {
                        let new_post = format!("{}\n{}\n", post.title, post.body);
                        respuesta.push_str(&new_post);
                    }

                    HttpResponse::Ok().body(respuesta)
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// #[post("/new_post")]
// async fn new_post(pool: web::Data<DbPool>) -> impl Responder {

// }
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let pool_connection = establish_pool_connection();

    HttpServer::new(move || {
        App::new()
            .service(index)
            .app_data(web::Data::new(pool_connection.clone()))
    }) // move es para mover ownership de pool_connection a la closure de HttpServer el hilo principal
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
