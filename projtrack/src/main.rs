use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod db;
mod routes;
mod auth;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let app = rocket::build()
        .manage(db::DbContext::new("sqlite:database.db").await)
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static/"));

    routes::mount(app).launch().await
}
