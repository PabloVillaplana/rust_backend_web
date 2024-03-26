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

    // self::insert_fake_data(conn);

    // Get specific post
    let search_id = 1;
    let results = get_specific_post(search_id, conn);

    // let results = posts
    //     .limit(100)
    //     .load::<Post>(conn)
    //     .expect("Error loading posts");

    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }

    println!("Hello, world!");
}

pub fn get_specific_post(search_id: i32, conn: &mut MysqlConnection) -> Vec<Post> {
    let specific_post = posts
        .filter(id.eq(search_id))
        .limit(1)
        .load::<Post>(conn)
        .expect("Error loading posts");

    return specific_post;
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
