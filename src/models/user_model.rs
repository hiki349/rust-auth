use chrono::{DateTime, Utc};

pub struct User {
    id: i64,
    email: String,
    passsword: String,
    created_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: i64, email: String, password: String, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            email,
            passsword: password,
            created_at,
        }
    }
}
