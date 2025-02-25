#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::status::Custom;
use rocket::http::Status;
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;

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
            MyError::DatabaseError(err) => {
                let error_message = format!("Database error: {}", err);
                Custom(Status::InternalServerError, error_message).respond_to(request)
            }
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

#[delete("/tasks/<id>")]
fn delete_task(conn: &rocket::State<DbConn>, id: i32) -> Result<(), MyError> {
    let conn = conn.0.lock().unwrap();
    conn.execute(
        "DELETE FROM task WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

#[put("/tasks/<id>/complete/<completed>")]
fn mark_task_done(conn: &rocket::State<DbConn>, id: i32, completed: bool) -> Result<(), MyError> {
    let conn = conn.0.lock().unwrap();
    conn.execute(
        "UPDATE task SET completed = ?1 WHERE id = ?2",
        params![completed as i32, id],
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
        .mount("/", routes![get_tasks, add_task, delete_task, mark_task_done])
        .mount("/", FileServer::from(relative!("static")))
}