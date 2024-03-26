#[derive(Queryable, Debug, Clone)]

pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub slug: String,
}

use super::schema::posts;

// Factory para crear nuevos posts
#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}
