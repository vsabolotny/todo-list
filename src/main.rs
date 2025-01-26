use std::io;

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("To-Do List:");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Mark task as done");
        println!("4. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => add_task(&mut tasks),
            2 => view_tasks(&tasks),
            3 => mark_task_done(&mut tasks),
            4 => break,
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter task description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    tasks.push(Task {
        description: description.trim().to_string(),
        completed: false,
    });

    println!("Task added!");
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        for (i, task) in tasks.iter().enumerate() {
            println!(
                "{}. {} [{}]",
                i + 1,
                task.description,
                if task.completed { "Done" } else { "Not done" }
            );
        }
    }
}

fn mark_task_done(tasks: &mut Vec<Task>) {
    println!("Enter task number to mark as done:");
    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number).unwrap();

    let task_number: usize = match task_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    if task_number == 0 || task_number > tasks.len() {
        println!("Invalid task number!");
    } else {
        tasks[task_number - 1].completed = true;
        println!("Task marked as done!");
    }
}