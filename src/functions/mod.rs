use std::io;
use crate::Task;
use crate::Status;

pub fn add_task(tasks: &mut Vec<Task>) {
    let mut description = String::new();
    println!("\n\nEnter the task description:");
    io::stdin().read_line(&mut description).expect("\n\nError reading input");

    let task = Task {
        description: description.trim().to_string(),
        status: Status::Pending,
    };
    tasks.push(task);
    println!("\n\nTask added successfully!");
}

pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("\n\nNo tasks available.");
        return;
    }
    for (i, task) in tasks.iter().enumerate() {
        let status = match task.status {
            Status::Pending => "[ ]",
            Status::Completed => "[✔]",
        };
        println!("\n\n {} {} - {}",i ,status ,task.description);
    }
}

pub fn mark_completed(tasks: &mut Vec<Task>) {
    list_tasks(tasks);
    println!("\n\nEnter the number of the task to mark as completed:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("\n\nError reading input");
    let index: usize = match input.trim().parse() {
        Ok(num) if num < tasks.len() => num,
        _ => {
            println!("\n\nInvalid number.");
            return;
        }
    };
    tasks[index].status = Status::Completed;
    println!("\n\nTask marked as completed!");
}