use serde::Serializer;
use sqlx::database::{HasValueRef, HasArguments};
use sqlx::error::BoxDynError;
use sqlx::{Database, Decode, Sqlite, Type, Encode};
use sqlx::encode::IsNull;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn from_plain(password: &str) -> Result<Self, argon2::password_hash::Error> {
        use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
        use argon2::Argon2;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hashed_password = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(Self(hashed_password))
    }
}

impl serde::Serialize for HashedPassword {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'r, DB: Database> Decode<'r, DB> for HashedPassword
where
    &'r str: Decode<'r, DB>,
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<DB>>::decode(value)?;
        Ok(Self(value.to_string()))
    }
}

impl<'r, DB: Database> Encode<'r, DB> for HashedPassword
where
    String: Encode<'r, DB>
{
    fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'r>>::ArgumentBuffer) -> IsNull {
        <String as Encode<DB>>::encode_by_ref(&self.0, buf)
    }
}

impl Type<Sqlite> for HashedPassword {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}
