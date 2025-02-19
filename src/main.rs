use rusqlite::{params, Connection, Result};
use todo_list::greet;
use std::io;

#[derive(Debug)]
#[allow(dead_code)]
struct Task {
    id: Option<i32>,
    description: String,
    completed: bool,
}

fn main() -> Result<()> {
    let conn = Connection::open("tasks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
                  id INTEGER PRIMARY KEY,
                  description TEXT NOT NULL,
                  completed INTEGER NOT NULL
                  )",
        [],
    )?;

    greet();

    loop {
        println!("To-Do List:");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Mark task as done");
        println!("4. Delete task");
        println!("5. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => add_task(&conn)?,
            2 => view_tasks(&conn)?,
            3 => mark_task_done(&conn)?,
            4 => delete_task(&conn)?,
            5 => break,
            _ => println!("Invalid choice! Please try again."),
        }
    }

    Ok(())
}

fn add_task(conn: &Connection) -> Result<()> {
    println!("Enter task description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    conn.execute(
        "INSERT INTO task (description, completed) VALUES (?1, ?2)",
        params![description.trim(), 0],
    )?;

    Ok(())
}

fn view_tasks(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, description, completed FROM task")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    for task in task_iter {
        let task = task?;
        println!("{:?}", task);
    }
    Ok(())
}

fn mark_task_done(conn: &Connection) -> Result<()> {
    println!("Enter the task number to mark as done:");
    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number).unwrap();
    let task_number: i32 = match task_number.trim().parse() {
        Ok(num) => num,
        Err(_) => return Ok(()),
    };

    conn.execute(
        "UPDATE task SET completed = 1 WHERE id = ?1",
        params![task_number],
    )?;

    Ok(())
}

fn delete_task(conn: &Connection) -> Result<()> {
    println!("Enter the task number to delete:");
    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number).unwrap();
    let task_number: i32 = match task_number.trim().parse() {
        Ok(num) => num,
        Err(_) => return Ok(()),
    };

    conn.execute(
        "DELETE FROM task WHERE id = ?1",
        params![task_number],
    )?;

    Ok(())
}
