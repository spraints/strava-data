use crate::token::Token;

pub fn new(_: Token) -> Client {
    Client {}
}

pub struct Client {}

impl Client {
    pub fn me(&self) -> anyhow::Result<String> {
        anyhow::bail!("todo");
    }
}
