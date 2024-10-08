pub struct Data {
    pub mongo_client: mongodb::Client,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;
