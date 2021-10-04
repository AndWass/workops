use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod auth;
mod db;
mod routes;

fn default_web_root<E>(_: E) -> Result<String, ()> {
    Ok(".".to_string())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();

    let web_root = std::env::var("WEB_ROOT").or_else(default_web_root).unwrap();
    let app = rocket::build()
        .manage(db::DbContext::new("sqlite:database.db").await)
        .attach(Template::fairing())
        .mount(
            "/static",
            FileServer::from(std::path::Path::new(&format!("{}/{}", web_root, "static/"))),
        );

    routes::mount(app).launch().await
}
