#![feature(decl_macro)]

extern crate rocket;
extern crate diesel;

use diesel::prelude::*;
use rocket_dyn_templates::{Template, context};
use dotenvy::dotenv;
use diesel::pg::PgConnection;
use std::env;

use rocket::tokio::time::{sleep, Duration};
use rocket::fs::{FileServer, relative, NamedFile};

use rocket::form::Form;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, launch, post, routes, FromForm};


mod models;
mod schema;

#[derive(Serialize, Deserialize)]
struct CreatedPost {
    title: String,
    body: String,

} 

#[derive(Serialize, Deserialize, FromForm)]
struct CreateForm<'r> {
    title: &'r str,
    body: &'r str,
}

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/post", format="json", data="<post>")]
fn create_post(post: Json<CreatedPost>) -> Result<Created<Json<CreatedPost>>> {
    use self::schema::repeat::dsl::*;
    use models::NewPost;
    let mut conn = establish_connection_pg();

    let new_post = NewPost {
        title: post.title.to_string(),
        body: post.body.to_string(),
        published: false,
    };

    diesel::insert_into(repeat)
        .values(&new_post)
        .execute(&mut conn)
        .expect("Error saving new post");

    Ok(Created::new("/").body(post))
}

#[get("/delete/<index>")]
fn delete_by_id(index:i32) -> Template{
    use self::schema::repeat::dsl::*;

    let connection = &mut establish_connection_pg();

    let _ = diesel::delete(repeat.filter(id.eq(index))).execute(connection);

    use self::models::Post;
    let connection = &mut establish_connection_pg();
    let results = self::schema::repeat::dsl::repeat
        .load::<Post>(connection)
        .expect("Error loading posts");
    Template::render("part_posts", context! {posts: &results, count: results.len()})
}

#[get("/")]
fn index() -> Template {
    use self::models::Post;
    let connection = &mut establish_connection_pg();
    let results = self::schema::repeat::dsl::repeat
        .load::<Post>(connection)
        .expect("Error loading posts");
    Template::render("index", context! {posts: &results, count: results.len()})
}

#[get("/admin")]
fn admin() -> Template {
    use self::models::Post;
    let connection = &mut establish_connection_pg();
    let results = self::schema::repeat::dsl::repeat
        .load::<Post>(connection)
        .expect("Error loading posts");
    Template::render("admin", context! {posts: &results, count: results.len()})
}

#[post("/admin/submit", data="<post>")]
fn submit_post(post: Form<CreateForm>) -> Template {
    use self::schema::repeat::dsl::*;
    use models::NewPost;
    let mut conn = establish_connection_pg();

    let res = CreatedPost{
        title: post.title.to_string(),
        body: post.body.to_string(),
    };


    let new_post = NewPost {
        title: post.title.to_string(),
        body: post.body.to_string(),
        published: false,
    };

    diesel::insert_into(repeat)
        .values(&new_post)
        .execute(&mut conn)
        .expect("Error saving new post");

    Template::render("create_ok", context! {post: &res,})
}

#[get("/part_post_layout")]
fn part_post_layout() -> Template {
    use self::models::Post;
    let connection = &mut establish_connection_pg();
    let results = self::schema::repeat::dsl::repeat
        .load::<Post>(connection)
        .expect("Error loading posts");
    Template::render("part_posts", context! {posts: &results, count: results.len()})
} 

#[get("/part_create_layout")]
fn part_create_layout() -> Template {
    use self::models::Post;
    let connection = &mut establish_connection_pg();
    let results = self::schema::repeat::dsl::repeat
        .load::<Post>(connection)
        .expect("Error loading posts");
    Template::render("part_create", context! {posts: &results, count: results.len()})
}

#[get("/boobs")]
fn boobs() -> String {
    "(.)(.)".to_string()  
}

#[get("/favicon.ico")]
async fn icon() -> Option<NamedFile> {
    NamedFile::open(relative!("favicon.avif")).await.ok()
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds!", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![admin])
        .mount("/", routes![boobs])
        .mount("/", routes![create_post])
        .mount("/", routes![submit_post])
        .mount("/", routes![delete_by_id])
        .mount("/", routes![part_post_layout])
        .mount("/", routes![part_create_layout])
        .mount("/", routes![delay])
        .mount("/", routes![icon])
        .mount("/public", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
