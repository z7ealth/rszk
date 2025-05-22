use std::fmt;

#[derive(Clone)]
pub struct Attendance {
    pub uid: u32,
    pub user_id: u32,
    pub timestamp: String,
    pub status: String,
    pub punch: i32,
}

impl Attendance {
    pub fn new(user_id: u32, timestamp: String, status: String, punch: i32, uid: u32) -> Self {
        Self {
            uid,
            user_id,
            timestamp,
            status,
            punch,
        }
    }
}

impl fmt::Display for Attendance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Attendance>: {} : {} ({}, {})",
            self.user_id, self.timestamp, self.status, self.punch
        )
    }
}

impl fmt::Debug for Attendance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Reuse Display implementation
        fmt::Display::fmt(self, f)
    }
}
