use prettytable::row;
#[macro_use]
// extern crate prettytable;
use prettytable::{Cell, Row, Table};


use std::{
    io::{Write, stdin, stdout},
    u16,
};


use crate::{account::Account, post::Post};

pub enum Command {
    Create,
    Delete,
    Update,
    View,
    Help,
    Unknow,
}
impl Command {
    pub fn new(command: &str) -> Command {
        match command {
            "create" => Command::Create,
            "delete" => Command::Delete,
            "update" => Command::Update,
            "view" => Command::View,
            "help" =>Command::Help,
            _ => Command::Unknow,
        }
    }
}
pub fn handle_command<'a>(raw_command: String, posts: &mut Vec<Post>, account: Account) {
    let v = raw_command.split(" ").collect::<Vec<&str>>();
    let command = Command::new(v[0]);
    match command {
        Command::Create => {
            print!("post>");
            stdout().flush().unwrap();
            let mut post = String::new();
            stdin()
                .read_line(&mut post)
                .expect("Error while trying to read from stdin");

            let current_post =
                Post::new((posts.len() + 1) as u16, account, post.trim().to_string());
            println!("Post Added {:?}", &current_post);
            posts.push(current_post);
        }
        Command::Delete => {
            println!("Delete command");
            let index: usize = v[1].parse().expect("error parsing post id");
            if index <= (posts.len()) {
                posts.remove(index - 1);
                println!("Deleted")
            } else {
                println!("Error deleting")
            }
        }
        Command::Update => {
            println!("Update command");
            let index: usize = v[1].parse().expect("error parsing post id");
            if index <= (posts.len()) {
                print!("post>");
                stdout().flush().unwrap();
                let mut post = String::new();
                stdin()
                    .read_line(&mut post)
                    .expect("Error while trying to read from stdin");

                let current_post = Post::new(index as u16, account, post.trim().to_string());
                posts.remove(index - 1);
                posts.insert(index - 1, current_post);
                println!("Updated")
            } else {
                println!("Error updating")
            }
        }
        Command::View => {
            let mut table = Table::new();
            table.add_row(row!["ID","USERNAME","POST"]);
            for p in posts{                
                table.add_row(
                  row![
                    p.id,p.account.username,p.post
                  ]  
                );
            }
            table.printstd();
        }
        Command::Help=>{
            println!("--This is instruction!
->help for more information
->create for create post
->delete id for delete with id(example usage. delete 3)
->update id for update with id(example usage. update 3)
->view for view all posts
->exit to exit the program")
        }
        _ => println!("Unknow Command"),
    }
}
