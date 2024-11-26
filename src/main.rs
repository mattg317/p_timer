use database::Database;
mod timer;
mod database;
// use crate::timer;



// fn main() -> Result<()> {
fn main() {
    println!("Hello, world!");
    // initialize_task_database().expect("Couldn't initialize dg");
    // view_tasks().expect("Couldn't find tasks")
    // let task_result = get_one_task().expect("Couldn't get task");
    let task_db = Database::new("./my_db.db3").expect("Error");
    task_db.insert_task().expect("Couldn't insert task");
    // let task = task_db.get_one_task(1).expect("Could get task");
    task_db.view_tasks().expect("Error")
    // println!("Task: {}", task[0]);
    // Ok(())


    // let user_task = timer::timer_setting();
    // run_alert()
    // list_tasks();
    // let (timer_length, timer_task) = timer::timer_settings();
    //     let user_task = timer::TimerSettings {
    //     length: timer_length,
    //     task: timer_task,
    // };
    // user_task.timer_setting();
    // timer::run_alert(&user_task);
}
