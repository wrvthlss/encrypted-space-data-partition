#[derive(Debug, Clone)]
pub enum AccessLevel {
    RegisteredUser,
    Guest,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub username: String,
    pub access_level: AccessLevel,
}

impl Session {
    /// Creates a new session for a registered user.
    pub fn new_registered(username: String) -> Self {
        Self {
            username,
            access_level: AccessLevel::RegisteredUser,
        }
    }

    /// Creates a new session for a guest user.
    pub fn new_guest(username: String) -> Self {
        Self {
            username,
            access_level: AccessLevel::Guest,
        }
    }

    /// Checks if the session belongs to a guest user.
    pub fn is_guest(&self) -> bool {
        matches!(self.access_level, AccessLevel::Guest)
    }

    /// Checks if the session belongs to a registered user.
    pub fn is_registered(&self) -> bool {
        matches!(self.access_level, AccessLevel::RegisteredUser)
    }
}
