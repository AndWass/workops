
use crate::db::DbContext;
use rocket::response::Debug;
use rocket::State;

type UiResult = Result<rocket_dyn_templates::Template, Debug<sqlx::Error>>;

#[derive(serde::Serialize)]
struct IndexContext {
    projects: Vec<crate::db::projects::Project>
}

#[rocket::get("/")]
async fn index(db: &State<DbContext>) -> UiResult {
    crate::db::projects::Project::all(db.executor()).await
        .map(|projects| {
            rocket_dyn_templates::Template::render("index", IndexContext {
                projects
            })
        })
        .map_err(|e| Debug(e))
}

pub fn routes() -> Vec<rocket::Route>
{
    rocket::routes![index]
}
