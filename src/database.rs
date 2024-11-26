use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Task {
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
        Ok( Database { conn: connection })

    }
    fn initialize_task_database(&self) -> Result<()> {
        // let path = "./my_db.db3";
        // let conn = Connection::open_in_memory()?;
        // let conn = Connection::open(path)?;

        self.conn.execute(
            "CREATE TABLE tasks (
            id    INTEGER PRIMARY KEY,
            task  TEXT,
            finished_task BOOL,
            created_date TIMESTAMP,
            time_spent INTEGER
        )",
            (), // empty list of parameters.
        )?;
        Ok(())
    }

    pub fn insert_task(&self) -> Result<()> {

        self.conn.execute(
            "INSERT INTO tasks (id, task, finished_taske, time_spent) VALUES (?1, ?2, ?3, ?4)",
            (2, "Feed dog", false, 10),
        )?;

        Ok(())
    }

    pub fn view_tasks(&self) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT id, task, time_spent FROM tasks")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                task: row.get(1)?,
                time_spent: row.get(2)?,
            })
        })?;

        for task in task_iter {
            let task_line = task.unwrap();
            println!("ID - {} Task - {}", task_line.id, task_line.task);
        }
        Ok(())
    }

    // TODO: implement for now to get one task and pash to the time
    pub fn get_one_task(&self, task_id: i32) -> Result<Vec<String>> {

        let mut stmt = self.conn.prepare("SELECT task FROM tasks WHERE id = :id")?;
        let rows = stmt.query_map(&[(":id", &task_id.to_string())], |row| row.get(0))?;

        let mut tasks = Vec::new();
        for task in rows {
            tasks.push(task?);
        }

        Ok(tasks)
    }
}
