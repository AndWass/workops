mod index;
mod project;

pub fn mount(base: &str, rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build>
{
    let mut routes = index::routes();
    routes.extend(project::routes());
    rocket.mount(base, routes)
}