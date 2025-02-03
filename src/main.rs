mod functions;

use std::io;
use functions::{add_task, list_tasks, mark_completed};

#[derive(Debug)]
enum Status {
    Pending,
    Completed,
}

#[derive(Debug)]
struct Task {
    description: String,
    status: Status,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark as Completed");
        println!("4. Exit");
        print!("Enter your choice: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("\n\nError reading input");
        let option: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\n\nPlease enter a valid number.");
                continue;
            }
        };

        match option {
            1 => add_task(&mut tasks),
            2 => list_tasks(&tasks),
            3 => mark_completed(&mut tasks),
            4 => {
                println!("\n\nExiting...");
                break;
            }
            _ => println!("\n\nInvalid option, please try again."),
        }
    }
}