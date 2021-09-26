use crate::db::Executor;
use rocket::serde::{Serialize, Deserialize};
use sqlx::{FromRow};

#[derive(Serialize, FromRow)]
pub struct User {
    id: i64,
    username: String,
    hashed_password: crate::auth::HashedPassword,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct NewUser
{
    pub username: String,
    pub password: String,
    pub email: String,
}

impl NewUser
{
    fn to_user(self, id: i64) -> User
    {
        User {
            id,
            username: self.username,
            hashed_password: crate::auth::HashedPassword::from_plain(self.password.as_str()).unwrap(),
            email: self.email,
            created_at: chrono::Utc::now(),
        }
    }
}

impl User {
    pub async fn all(executor: Executor<'_>) -> sqlx::Result<Vec<User>> {
        sqlx::query_as("SELECT * FROM User")
            .fetch_all(executor)
            .await
    }

    pub async fn get(id: i64, executor: Executor<'_>) -> sqlx::Result<User> {
        sqlx::query_as("SELECT * FROM User WHERE id=?")
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn create(new_user: NewUser, executor: Executor<'_>) -> sqlx::Result<User>
    {
        let mut user = new_user.to_user(0);
        let res: sqlx::sqlite::SqliteQueryResult = sqlx::query(
            r#"INSERT INTO User (username, hashed_password, email, created_at)
                    VALUES (?,?,?,?)"#)
            .bind(&user.username)
            .bind(&user.hashed_password)
            .bind(&user.email)
            .bind(&user.created_at)
            .execute(executor)
            .await?;
        user.id = res.last_insert_rowid();
        Ok(user)
    }

    pub async fn find(username: &str, executor: Executor<'_>) -> sqlx::Result<Option<User>>
    {
        sqlx::query_as("SELECT * FROM User WHERE username=?")
            .bind(username)
            .fetch_optional(executor)
            .await
    }
}
