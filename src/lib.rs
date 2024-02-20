use std::string::ToString;
use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;

pub struct Task {
    id: usize,
    description: String,
    date: usize,
    expires: usize,
    state: bool,
}

pub struct Command {
    id: u32,
    description: &'static str,
    call_argument: &'static str,
}

impl Command {
    pub const fn new(id: u32, description: &'static str, call_argument: &'static str) -> Self {
        Command { id, description, call_argument }
    }

    pub fn get_by_id(id: u32, command_list: &'static [Self]) -> Option<&'static Self> {
        for command in command_list {
            if command.id == id {
                return Some(command);
            }
        }
        None
    }

    pub fn get_by_call(call: &'static str, command_list: &'static [Self]) -> Option<&'static Self> {
        for command in command_list {
            if command.call_argument == call {
                return Some(command);
            }
        }
        None
    }

    pub fn get_action() -> () {  // -> Result<>

    }
}

pub const COMMANDS: &[Command] = &[
    Command::new(1, "Add task", "add"),
    Command::new(2, "View tasks", "view"),
    Command::new(3, "Exit","exit"),
];
impl Task {

    fn print_commands(commands: &[Command]) {
        for cmd in commands {
            println!("{}. {} ({})", cmd.id, cmd.description, cmd.call_argument);
        }
    }

    pub fn get_command(args: Vec<String>) -> Option<&'static Command> {
        if (args.len() == 1) {
            // Show the select menu
            loop {
                Self::print_commands(COMMANDS);
                /*
                let choice: usize = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Choose an option")
                    .interact_text()
                    .unwrap();
                if (choice > 0 && choice <= COMMANDS.len()) {
                    return choice;
                 */
                let choice = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("Choose an option (1-{})", COMMANDS.len()))
                    .validate_with(move |input: &String| {
                        match input.parse::<usize>() {
                            Ok(choice) if choice > 0 && choice <= COMMANDS.len() => Ok::<(), _>(()),
                            _ => Err("Invalid choice".to_string()),
                        }
                    })
                    .interact()
                    .unwrap();
                match choice.parse::<u32>() {
                    Ok(command_id) => {
                        return Command::get_by_id(command_id, COMMANDS); // .unwrap()
                    }
                    Err(_) => {
                        println!("Error: The string cannot be converted to a u32. This SHOULDN'T\
                        happen. Please submit an issue on GitHub if this keeps occuring.");
                    }
                }
                }
            }
        else {
            // Match existing commands
            let args_command = &args[1].to_owned();
            //println!("{}", args_command);
            return Command::get_by_call(args_command, COMMANDS);
        }
    }
    pub fn new() {

    }
}