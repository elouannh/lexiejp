use std::error::Error as StdError;

type Error = Box<dyn StdError + Send + Sync>;
