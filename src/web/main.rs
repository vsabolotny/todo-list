#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::{Serialize, json::Json};
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;
use std::io;

#[derive(Debug, Serialize)]
struct Task {
    id: Option<i32>,
    description: String,
    completed: bool,
}

struct DbConn(Mutex<Connection>);

#[get("/tasks")]
fn get_tasks(conn: &rocket::State<DbConn>) -> Json<Vec<Task>> {
    let conn = conn.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, description, completed FROM task").unwrap();
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    }).unwrap();

    let tasks: Vec<Task> = task_iter.map(|task| task.unwrap()).collect();
    Json(tasks)
}

#[post("/tasks", format = "json", data = "<task>")]
fn add_task(conn: &rocket::State<DbConn>, task: Json<Task>) -> Result<()> {
    let conn = conn.0.lock().unwrap();
    conn.execute(
        "INSERT INTO task (description, completed) VALUES (?1, ?2)",
        params![task.description, task.completed as i32],
    )?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    let conn = Connection::open("tasks.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
                  id INTEGER PRIMARY KEY,
                  description TEXT NOT NULL,
                  completed INTEGER NOT NULL
                  )",
        [],
    ).unwrap();

    rocket::build()
        .manage(DbConn(Mutex::new(conn)))
        .mount("/", routes![get_tasks, add_task])
        .mount("/", FileServer::from(relative!("static")))
}