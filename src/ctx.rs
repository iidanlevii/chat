#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
}

impl Ctx {
    pub fn new(user_id: i64) -> Self {
        Self { user_id }
    }
}

impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
