use crate::account::{Account};
#[derive(Debug)]
pub struct Post {
    pub id: u16,
    pub account: Account,
    pub post: String,
}
impl Post {
    pub fn new(id: u16, account: Account, post: String) -> Post {
        Post {
            id: id,
            account: account,
            post: post,
        }
    }
}
