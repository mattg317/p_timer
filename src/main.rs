use database::Database;
use std::io;
use std::path::Path;
mod database;
mod timer;
// use crate::timer;

fn print_menu() -> String {
    println!("Welcome to Task Doer");
    println!("Please Select from the Menu");
    println!("[L]ist tasks - [A]dd Task - [Q]uit Program - [W]ork on a task");

    //
    let mut menu_choice = String::new();
    io::stdin()
        .read_line(&mut menu_choice)
        .expect("Failed to read line");
    let menu_choice: String = menu_choice.trim().to_string().to_lowercase();

    println!("You selected {}", menu_choice);
    menu_choice
}
// fn main() -> Result<()> {
fn main() {
    println!("Hello, world!");
    // =============== DATABASE TEsting ==========================
    // 1. Check to initialize db
    let file_path: String = "./my_db.db3".to_string();

    if Path::new(&file_path).exists() != true {
        database::initialize_task_database().expect("Error"); // should I have this be it's own
                                                              // function?
    }
    let task_db = Database::new(&file_path).expect("Error");

    // Prompt menu
    // MAIN program
    // let mut menu_loop: bool = true;
    // while menu_loop == true {
    //     let menu_choice = print_menu();
    //     match menu_choice.as_str() {
    //         "l" => task_db.view_tasks().expect("Error"),
    //         "a" => task_db.insert_task().expect("Error"),
    //         // "q" => println!("good bye!"),
    //         "q" => menu_loop = false,
    //         _ => println!("Not a valid choice"),
    //     }
    // }

    // task_db.insert_task().expect("Couldn't insert task");
    let task = task_db.get_one_task().expect("Couldn't get task");
    // task_db.view_tasks().expect("Error")
    println!("Task: {}", task[0]);

    // Run task from DB
    let user_task = timer::TimerSettings {
        length: 5,
        task: task[0],
    };
    timer::run_alert(&user_task)

    // =============== Menu TEsting ==========================
}
