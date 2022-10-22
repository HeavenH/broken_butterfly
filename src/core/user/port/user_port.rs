use async_trait::async_trait;

#[async_trait(?Send)]
pub trait UserPort {
    async fn create_user(&self) -> Result<User, Error>;
}