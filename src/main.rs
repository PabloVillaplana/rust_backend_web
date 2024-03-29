#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema; // Modelos

// use actix_web::post;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use platzi_rust_blog_post::establish_pool_connection;
use tera::Tera;

use self::models::Post;
use self::schema::posts::dsl::*;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
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

                    let data = data.unwrap();
                    for post in &data {
                        let new_post = format!("{}\n{}\n", post.title, post.body);
                        respuesta.push_str(&new_post);
                    }

                    let mut ctx = tera::Context::new();
                    ctx.insert("posts", &data);

                    HttpResponse::Ok().body(template_manager.render("index.html", &ctx).unwrap())
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// #[post("/new_post")]
// async fn add_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
//     match pool.get() {
//         Ok(conn) => {
//             match web::block(move || {
//                 let mut conn = conn;
//                 Post::create_post(&mut conn, &item)
//             })
//             .await
//             {
//                 Ok(_) => HttpResponse::Ok().finish(),
//                 Err(_) => HttpResponse::InternalServerError().finish(),
//             }
//         }
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let pool_connection = establish_pool_connection();

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .service(index)
            // .service(add_post)
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(pool_connection.clone()))
    }) // move es para mover ownership de pool_connection a la closure de HttpServer el hilo principal
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
