use crate::{client::Clients, types::Result};

#[derive(Debug)]
pub struct State {
    pub filename: String,

    pub clients: Clients,
}

impl State {
    pub fn new<T: AsRef<str>>(filename: T) -> Result<Self> {
        Ok(State {
            filename: filename.as_ref().to_string(),
            clients: Clients::new(),
        })
    }
    
    pub fn filename(&self) -> &str {
        self.filename.as_str()
    }
}
