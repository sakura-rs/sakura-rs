use std::sync::LazyLock;

use regex::Regex;
use sqlx::{Database, Decode, Encode, Postgres};

#[derive(Encode, Decode)]
pub struct Username(String);

impl Username {
    pub fn parse(username: String) -> Option<Self> {
        static ALLOWED_PATTERN: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("^[a-zA-Z0-9._@-]{6,25}$").unwrap());

        ALLOWED_PATTERN
            .is_match(&username)
            .then_some(Self(username))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl sqlx::Type<Postgres> for Username {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }

    fn compatible(ty: &<Postgres as Database>::TypeInfo) -> bool {
        <String as sqlx::Type<Postgres>>::compatible(ty)
    }
}
