use crate::db::Executor;
use rocket::serde::{Serialize, Deserialize};
use sqlx::{FromRow};

#[derive(Serialize, FromRow, PartialEq, Eq, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub hashed_password: crate::auth::HashedPassword,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
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

    pub async fn get(id: i64, executor: Executor<'_>) -> sqlx::Result<Option<User>> {
        sqlx::query_as("SELECT * FROM User WHERE id=?")
            .bind(id)
            .fetch_optional(executor)
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

#[cfg(test)]
mod tests
{
    use rocket::tokio;

    use super::*;
    use crate::db::in_memory_db_context;

    #[tokio::test]
    async fn create_user() {
        let db =  in_memory_db_context().await;

        let new_user = NewUser {
            username: "testuser".to_string(),
            password: "abc123".to_string(),
            email: "xyz".to_string(),
        };

        let new_user = User::create(new_user, db.executor()).await.unwrap();

        assert!(new_user.id > 0);
        assert_eq!(new_user.username, "testuser");
        assert_eq!(new_user.email, "xyz");

        let user_by_id = User::get(new_user.id, db.executor()).await.unwrap().unwrap();
        assert_eq!(user_by_id, new_user);

        let found_user = User::find("testuser", db.executor()).await.unwrap().unwrap();
        assert_eq!(found_user, new_user);
    }
}
