use std::error::Error;

pub trait Repository {
    type DTO;
    fn next_identity(&self) -> String;
    fn create(&self, data: Self::DTO) -> Result<bool, Box<dyn Error>> {
        unimplemented!()
    }
    async fn async_create(&self, data: Self::DTO) -> Result<bool, Box<dyn Error>> {
        unimplemented!()
    }
}
