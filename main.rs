use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};

const FILE_NAME: &str = "todo.txt";

fn main() {
    loop {
        println!("\nTo-Do List:");
        list_tasks();

        println!("\nOptions:");
        println!("1. Add Task");
        println!("2. Remove Task");
        println!("3. Exit");

        let choice = get_input("Choose an option: ");

        match choice.trim() {
            "1" => add_task(),
            "2" => remove_task(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Try again."),
        }
    }
}

fn list_tasks() {
    match read_to_string(FILE_NAME) {
        Ok(content) => {
            if content.is_empty() {
                println!("(No tasks yet)");
            } else {
                for (i, line) in content.lines().enumerate() {
                    println!("{}. {}", i + 1, line);
                }
            }
        }
        Err(_) => println!("(No tasks file found)"),
    }
}

fn add_task() {
    let task = get_input("Enter a new task: ");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILE_NAME)
        .expect("Failed to open file");

    writeln!(file, "{}", task).expect("Failed to write to file");
    println!("Task added!");
}

fn remove_task() {
    let tasks = match read_to_string(FILE_NAME) {
        Ok(content) => content.lines().map(String::from).collect::<Vec<String>>(),
        Err(_) => {
            println!("No tasks found.");
            return;
        }
    };

    if tasks.is_empty() {
        println!("No tasks to remove.");
        return;
    }

    let index = get_input("Enter task number to remove: ")
        .trim()
        .parse::<usize>()
        .unwrap_or(0);

    if index == 0 || index > tasks.len() {
        println!("Invalid task number.");
        return;
    }

    let new_tasks: Vec<_> = tasks.iter().enumerate()
        .filter(|(i, _)| i + 1 != index)
        .map(|(_, t)| t.clone()) // Clone to retain ownership
        .collect();

    std::fs::write(FILE_NAME, new_tasks.join("\n")).expect("Failed to update file");
    println!("Task removed!");
}


fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
