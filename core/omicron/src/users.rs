#[derive(Debug)]
pub struct UserId(pub u64);

#[derive(Debug)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub created_at: String,
}
