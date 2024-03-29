use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub slug: String,
}

// use crate::schema::posts::{slug, title};

use super::schema::posts;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}

// Serializador
#[derive(Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
    pub slug: String,
}

//Funciones Implementadas

// use diesel::mysql::MysqlConnection;
// use diesel::{connection, prelude::*};

impl Post {
    pub fn slugify(other_title: &String) -> String {
        return other_title
            .to_lowercase()
            .replace(" ", "-")
            .replace("?", "")
            .replace("¿", "")
            .replace("¡", "")
            .replace("!", "")
            .replace(":", "")
            .replace(";", "")
            .replace(",", "")
            .replace(".", "")
            .replace("á", "a")
            .replace("é", "e")
            .replace("í", "i")
            .replace("ó", "o")
            .replace("ú", "u")
            .replace("ñ", "n");
    }

    // pub fn create_post<'a>(
    //     connection: &mut MysqlConnection,
    //     post: &NewPostHandler,
    // ) -> Result<Post, diesel::result::Error> {
    //     let other_slug = Post::slugify(&post.title);

    //     let new_post = NewPost {
    //         title: &post.title.as_str(),
    //         body: &post.body.as_str(),
    //         slug: &other_slug,
    //     };

    //     return diesel::insert_into(posts::table)
    //         .values(new_post)
    //         .get_result::<Post>(connection);
    // }
}
