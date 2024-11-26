use std::io;
use std::process::Command;
use std::{thread, time};

pub struct TimerSettings {
    pub length: u64,
    pub task: String,
}


pub fn run_alert(user_task: &TimerSettings) {
    println!("Running timer");
    println!("Time set: {} For: {}", user_task.length, user_task.task);
    thread::sleep(time::Duration::from_secs(user_task.length));
    let output = Command::new("notify-send")
        .arg(&user_task.task)
        .output()
        .expect("failed to execute");
    output.stdout;
}

// this will implement the logic from the sqlite stuff
pub fn list_tasks() {
    let task = vec!["Task 1", "Task 2", "Task 3"];
    let mut num = 1;
    for i in &task {
        println!("{} - {}", num, i);
        num += 1;
    }

    println!("Select a task");
    let mut task_num = String::new();
    io::stdin()
        .read_line(&mut task_num)
        .expect("Failed to read line");
    let task_num: usize = task_num.trim().parse().expect("Please type a number!");

    // Will need error handling here
    println!("You select {}", &task[task_num - 1]);
}

pub fn timer_settings() -> (u64, String) {
    // Get Length
    println!("How long to set the timer for?");
    let mut timer_length = String::new();
    io::stdin()
        .read_line(&mut timer_length)
        .expect("Failed to read line");
    let timer_length: u64 = timer_length.trim().parse().expect("Please type a number!");

    // Get Task
    println!("What task is the timer for?");
    let mut timer_task = String::new();
    io::stdin()
        .read_line(&mut timer_task)
        .expect("Failed to read line");
    let timer_task: String = timer_task.trim().to_string();

    // Let return a tuple instead then the struct will get initialized later
    (timer_length, timer_task)
}
