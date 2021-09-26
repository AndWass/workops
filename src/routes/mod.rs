use crate::db::users::User;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::serde::{Serialize};

type DbContext = rocket::State<crate::db::DbContext>;
type ApiResult<T> = Result<Json<T>, Debug<sqlx::Error>>;

#[derive(Serialize)]
struct AllUsers {
    users: Vec<User>,
}

#[rocket::get("/<id>")]
async fn get_user(id: i64, db: &DbContext) -> ApiResult<User> {
    User::get(id, db.executor())
        .await
        .map(|x| Json(x))
        .map_err(|e| Debug(e))
}

#[rocket::get("/list")]
async fn list_users(db: &DbContext) -> ApiResult<AllUsers> {
    User::all(db.executor())
        .await
        .map(|x| Json(AllUsers { users: x }))
        .map_err(|e| Debug(e))
}

#[rocket::post("/create", data = "<user>")]
async fn create_user(user: Json<crate::db::users::NewUser>, db: &DbContext) -> ApiResult<User> {
    User::create(user.into_inner(), db.executor())
        .await
        .map(|x| Json(x))
        .map_err(|e| Debug(e))
}

pub fn mount(app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    app.mount("/user", rocket::routes![get_user, list_users, create_user])
}
