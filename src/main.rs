use std::io::{Write, stdin, stdout};

use account::Account;
use command::handle_command;
use post::Post;
mod account;
mod command;
mod post;
fn main() {
    let welcome_text = "Welcome to My Blog";
    println!("{}", welcome_text);
    let mut posts: Vec<Post> = Vec::new();
    let mut my_account: Option<Account> = None;
    loop {
        let mut command = String::new();
        if my_account.is_none() {
            println!("Create Account");
            stdout().flush().unwrap();
            let mut username = String::new();
            let mut name = String::new();
            print!("username> ");
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut username)
                .expect("Error while trying to read from stdin");
            print!("name> ");
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut name)
                .expect("Error while trying to read from stdin");
            let account = Account::new(username.trim().to_string(), name.trim().to_string());
            println!("Account has been created for {}", account.get_username());
            let _ = my_account.insert(account);
            println!(
                "--This is instruction!
->help for more information
->create for create post
->delete id for delete with id(example usage. delete 3)
->update id for update with id(example usage. update 3)
->view for view all posts
->exit to exit the program"
            )
        }
        print!("blog> ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut command)
            .expect("Error while trying to read from stdin");
        if command.trim() == "exit" {
            println!("Exiting the program. Bye Bye");
            break;
        }
        handle_command(
            command.trim().to_string(),
            &mut posts,
            my_account.clone().unwrap(),
        );
    }
}
