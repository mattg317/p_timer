use rusqlite::{Connection, Result};
use std::io;

#[derive(Debug)]
pub struct Task {
    id: i32,
    task: String,
    time_spent: i32,
}

pub struct Database {
    pub conn: Connection,
}
impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let connection = Connection::open(db_path)?;
        Ok(Database { conn: connection })
    }

    pub fn insert_task(&self) -> Result<()> {
        println!("What Task would you like to add?");
        let mut task_add = String::new();
        io::stdin()
            .read_line(&mut task_add)
            .expect("Failed to read line");
        let task_add: String = task_add.trim().to_string();

        self.conn.execute(
            "INSERT INTO tasks (task, finished_task, created_date, time_spent) VALUES (?1, ?2, CURRENT_TIMESTAMP, ?3)",
            (&task_add, false, 0),
        )?;
        println!("Added task {}", task_add);

        Ok(())
    }

    pub fn view_tasks(&self) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, task, time_spent FROM tasks")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                task: row.get(1)?,
                time_spent: row.get(2)?,
            })
        })?;

        for task in task_iter {
            let task_line = task.unwrap();
            println!("[{}] - {}", task_line.id, task_line.task);
        }
        Ok(())
    }

    // TODO: implement for now to get one task and pash to the time
    pub fn get_one_task(&self) -> Result<Vec<String>> {
        println!("Which task would you like to work on?");
        let _ = self.view_tasks();

        let mut task_choice = String::new();
        io::stdin()
            .read_line(&mut task_choice)
            .expect("Failed to read line");
        let task_choice: String = task_choice.trim().to_string();

        let mut stmt = self.conn.prepare("SELECT task FROM tasks WHERE id = :id")?;
        let rows = stmt.query_map(&[(":id", &task_choice)], |row| row.get(0))?;

        let mut tasks = Vec::new();
        for task in rows {
            tasks.push(task?);
        }

        Ok(tasks)
    }
}

pub fn initialize_task_database() -> Result<()> {
    let path = "./my_db.db3";
    let conn = Connection::open(path)?;

    // TODO: redo table for id to be autoincremented
    conn.execute(
        "CREATE TABLE tasks (
            id    INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            task  TEXT,
            finished_task BOOL,
            created_date TIMESTAMP,
            time_spent INTEGER
        )",
        (), // empty list of parameters.
    )?;
    Ok(())
}
