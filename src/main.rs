use console::Style;
use dialoguer::{Input, Confirm, Editor, Select, theme::ColorfulTheme};
use todo_rs::{Command, Task};
use std::env;

fn main() {



    let args: Vec<String> = env::args().collect();
    let matched_command = Task::get_command(args);
    //println!("{}", matched_command);

}