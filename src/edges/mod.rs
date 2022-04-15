pub mod https;

pub struct Client<'a> {
    c: &'a crate::Client,
}

impl<'a> Client<'a> {
    pub fn new(c: &'a crate::Client) -> Self {
        Self{c}
    }
    pub fn https(&self) -> https::Client {
        https::Client::new(self.c)
    }
}
