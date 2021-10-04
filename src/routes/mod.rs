
mod api;
mod ui;

pub fn mount(app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let rocket = api::mount("/_api/v1", app);
    let rocket = ui::mount("/", rocket);
    rocket
}
