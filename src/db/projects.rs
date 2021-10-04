use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, PartialEq, Eq, Debug)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub created_at: crate::db::DateTime,
    owner: i64,
}

impl Project {
    pub async fn all(e: crate::db::Executor<'_>) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as("SELECT id, name, description, created_at, owner FROM project ORDER BY id")
            .fetch_all(e)
            .await
    }

    pub async fn get(id: i64, e: crate::db::Executor<'_>) -> sqlx::Result<Option<Self>> {
        sqlx::query_as("SELECT id, name, description, created_at, owner FROM project WHERE id=?")
            .bind(id)
            .fetch_optional(e)
            .await
    }
}

#[derive(Serialize, FromRow, PartialEq, Eq, Debug)]
pub struct NewProject {
    pub owner: i64,
    pub name: String,
    pub description: String,
}

impl NewProject {
    pub async fn insert(self, e: crate::db::Executor<'_>) -> sqlx::Result<Option<Project>> {
        let result: sqlx::sqlite::SqliteQueryResult = sqlx::query(
            r#"INSERT INTO project (name, description, created_at, owner) VALUES(?, ?, datetime('now'), ?)"#)
            .bind(self.name)
            .bind(self.description)
            .bind(self.owner)
            .execute(e)
            .await?;

        Project::get(result.last_insert_rowid(), e).await
    }
}

#[cfg(test)]
mod tests {
    use rocket::tokio;

    #[tokio::test]
    async fn insert() {
        let db = crate::db::in_memory_db_context().await;
        let before_creation = chrono::Utc::now();

        let new = super::NewProject {
            owner: 1,
            name: "Test project".to_string(),
            description: "Some description".to_string(),
        }
        .insert(db.executor())
        .await
        .unwrap()
        .unwrap();

        let new = super::Project::get(new.id,  db.executor()).await.unwrap().unwrap();

        assert_eq!(new.name, "Test project");
        assert_eq!(new.description, "Some description");
        assert_eq!(new.owner, 1);
        assert!(new.created_at >= (before_creation -chrono::Duration::seconds(1)));
    }
}
