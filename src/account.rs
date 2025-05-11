#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub username: String,
    pub name: String,
}
impl Account {
    pub fn new(username: String, name: String) -> Account {
        Account {
            username: username,
            name: name,
        }
    }
    // pub fn set_username(&mut self, new_username: String) {
    //     self.username = new_username
    // }
    pub fn get_username(&self) -> &String {
        &self.username
    }
}
