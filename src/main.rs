#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema; // Modelos

use diesel::prelude::*;
use platzi_rust_blog_post::establish_connection;

use self::models::{NewPost, Post};
use self::schema::posts;
use self::schema::posts::dsl::*;

fn main() {
    let conn = &mut establish_connection();

    // Insert fake data
    // self::insert_fake_data(conn);

    // Get specific post
    let search_id = 1;
    get_specific_post(search_id, conn);

    // Edit record
    let search_id = 6;
    edit_record(search_id, conn);

    // Delete record
    let search_id = 6;
    delete_record(search_id, conn);

    // Get All Post
    get_all_post(conn);
}

pub fn delete_record(search_id: i32, conn: &mut MysqlConnection) {
    // Esta operacion puede devolver la cantidad de registros eliminados
    diesel::delete(posts.find(search_id))
        .execute(conn)
        .expect("Error deleting post");
}

pub fn edit_record(search_id: i32, conn: &mut MysqlConnection) {
    diesel::update(posts.find(search_id))
        .set(title.eq("Empanadas"))
        .execute(conn)
        .expect("Error updating post");
}

pub fn get_specific_post(search_id: i32, conn: &mut MysqlConnection) -> Vec<Post> {
    let specific_post = posts
        .filter(id.eq(search_id))
        .limit(1)
        .load::<Post>(conn)
        .expect("Error loading posts");

    return specific_post;
}

pub fn get_all_post(conn: &mut MysqlConnection) {
    let results = posts
        .limit(6)
        .load::<Post>(conn)
        .expect("Error loading posts");

    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}

pub fn insert_fake_data(conn: &mut MysqlConnection) {
    let new_post = vec![
        NewPost {
            title: "Hello",
            body: "World",
            slug: "hello-world",
        },
        NewPost {
            title: "Hello",
            body: "World",
            slug: "hello-world",
        },
        NewPost {
            title: "Hello",
            body: "World",
            slug: "hello-world",
        },
    ];

    diesel::insert_into(posts::table)
        .values(new_post)
        .execute(conn)
        .expect("Error saving new post");
}
