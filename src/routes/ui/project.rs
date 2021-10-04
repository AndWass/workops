use rocket::{get, post, response::Redirect};
use rocket_dyn_templates::Template;
use rocket::form::FromForm;

#[get("/project/new")]
fn new() -> Template {
    #[derive(Default, serde::Serialize)]
    struct NewProjectContext {
        pub parent: Option<i64>,
    }

    Template::render("project/new_project", NewProjectContext::default())
}

#[derive(Default, FromForm)]
struct NewProject {
    name: String,
    description: String,
}

#[post("/project/new", data="<input>")]
fn create_new(input: rocket::form::Form<NewProject>) -> rocket::response::Redirect {
    println!("Creating project {}: {}", input.name, input.description);
    Redirect::to(rocket::uri!("/"))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![new, create_new]
}
