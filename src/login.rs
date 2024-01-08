use crate::client;
use crate::token;

pub fn main() {
    match token::read() {
        Ok(token) => match client::new(token).me() {
            Ok(me) => println!("{me} is already logged in."),
            Err(_) => do_login(),
        },
        Err(_) => do_login(),
    }
}

pub fn do_login() {
    todo!()
}
