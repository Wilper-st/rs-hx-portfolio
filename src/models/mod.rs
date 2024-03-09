use super::schema::repeat;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = repeat)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = repeat)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}
