use rand::Rng;
use sqlx::sqlite;
use sqlx::sqlite::SqlitePoolOptions;

pub mod users;
pub mod projects;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub type Executor<'a> = &'a sqlite::SqlitePool;
pub type DateTime = chrono::DateTime<chrono::Utc>;

pub struct DbContext {
    pool: sqlite::SqlitePool,
}

impl DbContext {
    pub async fn new(uri: &str) -> Self {
        const ADMIN_USERNAME: &'static str = "admin";

        let pool = SqlitePoolOptions::new()
            .connect(uri)
            .await
            .unwrap();

        MIGRATOR.run(&pool).await.unwrap();

        let admin_user = users::User::find(ADMIN_USERNAME, &pool).await.unwrap();

        if admin_user.is_none() {
            let mut admin_password = [0; 24];
            for (index, value) in rand::rngs::OsRng
                .sample_iter(rand::distributions::Alphanumeric)
                .enumerate()
                .take(admin_password.len())
            {
                admin_password[index] = value;
            }

            // SAFETY: all bytes are alphanumeric so they are valid UTF-8
            let admin_password = unsafe { std::str::from_utf8_unchecked(&admin_password) };
            println!("!!! Adding administrator username and password:\n!!!\tUsername: {}\n!!!\tPassword: {}", ADMIN_USERNAME, admin_password);
            users::NewUser {
                username: ADMIN_USERNAME.to_string(),
                password: admin_password.to_string(),
                email: "root@root.local".to_string(),
            }.insert(&pool)
            .await
            .unwrap();
        }

        Self { pool }
    }

    pub fn executor(&self) -> &sqlite::SqlitePool {
        &self.pool
    }
}

#[cfg(test)]
pub(crate) async fn in_memory_db_context() -> DbContext {
    DbContext::new("sqlite::memory:").await
}
