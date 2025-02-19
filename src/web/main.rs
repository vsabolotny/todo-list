#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::status::Custom;
use rocket::http::Status;
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: Option<i32>,
    description: String,
    completed: bool,
}

struct DbConn(Mutex<Connection>);

#[derive(Debug)]
enum MyError {
    DatabaseError(rusqlite::Error),
}

impl<'r> rocket::response::Responder<'r, 'static> for MyError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            MyError::DatabaseError(_) => Custom(Status::InternalServerError, "Database error").respond_to(request),
        }
    }
}

impl From<rusqlite::Error> for MyError {
    fn from(err: rusqlite::Error) -> MyError {
        MyError::DatabaseError(err)
    }
}

#[get("/tasks")]
fn get_tasks(conn: &rocket::State<DbConn>) -> Result<Json<Vec<Task>>, MyError> {
    let conn = conn.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, description, completed FROM task")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let tasks: Vec<Task> = task_iter.map(|task| task.unwrap()).collect();
    Ok(Json(tasks))
}

#[post("/tasks", format = "json", data = "<task>")]
fn add_task(conn: &rocket::State<DbConn>, task: Json<Task>) -> Result<(), MyError> {
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