pub mod https;

pub struct Client {
    c: crate::Client,
}

impl Client {
    pub fn new(c: crate::Client) -> Self {
        Self { c }
    }
    pub fn https(&self) -> https::Client {
        https::Client::new(self.c.clone())
    }
}
