use uuid::Uuid;
use std::{io::Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::fs::{self, OpenOptions, File};
use inquire::{Select, Text, DateSelect};
use chrono::{DateTime, Local, NaiveDate, Utc};

#[derive(Debug ,Serialize, Deserialize, Clone, PartialEq)]
enum Status {
    FAILED,
    PENDING,
    COMPLETED
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Habit {
    id: Uuid,
    name: String,
    status: Status,
    create_at: DateTime<Utc>,
    time_limit: NaiveDate
}

#[derive(Serialize, Deserialize)]
struct HabitCompletHistory {
    id: Uuid,
    habit_id: Uuid,
    status_alterate: Status,
    update_at: DateTime<Utc>
}

struct ReturnLoadData {
    habits_list: Vec<Habit>,
    habits_history_list: Vec<HabitCompletHistory>
}

fn refresh_status(habits: &mut Vec<Habit>) {
    let today: NaiveDate = Local::now().date_naive();

    for habit in habits.iter_mut() {
        if habit.status == Status::PENDING && today > habit.time_limit {
            habit.status = Status::FAILED;
        }
    }
}

async fn create_files(path: &str, path_complet_history: &str) {
    if Path::new(path).exists() {
        File::open(path).unwrap()
    } else {
        File::create(path).unwrap()
    };

    if Path::new(path_complet_history).exists() {
        File::open(path_complet_history).unwrap()
    } else {
        File::create(path_complet_history).unwrap()
    };
}

async fn load_habits(path_habit: &str, path_complet_history: &str) -> ReturnLoadData {

    let mut habits_list: Vec<Habit>;
    let habits_complet_history_list: Vec<HabitCompletHistory>;

    if Path::new(path_habit).exists() {
        let content_file = fs::read_to_string(path_habit).expect("Failed in read file");

        if content_file.trim().is_empty() {
            habits_list = Vec::new()
        } else {
            habits_list = serde_json::from_str(&content_file).expect("Filed in convert JSON old")
        }
    } else {
        habits_list = Vec::new()
    };
    
    if Path::new(path_complet_history).exists() {
        let content_file = fs::read_to_string(path_complet_history).expect("Failed in read file");
        
        if content_file.trim().is_empty() {
           habits_complet_history_list = Vec::new()
        } else {
            habits_complet_history_list = serde_json::from_str(&content_file).expect("Filed in convert JSON old")
        }
    } else {
        habits_complet_history_list = Vec::new()
    };

    refresh_status(&mut habits_list);

    let return_data = ReturnLoadData {
        habits_list: habits_list,
        habits_history_list: habits_complet_history_list
    };

    return_data
}

#[tokio::main]
async fn main() {

    let path_habit = "habit.json";
    let path_complet_history = "habit_complet_hitory.json";

    create_files(path_habit, path_complet_history).await;

    let data = load_habits(path_habit, path_complet_history).await;

    let mut habits_list = data.habits_list;
    let mut habits_complet_history_list = data.habits_history_list;

    let list = vec![
        "Create new habit",
        "Edit habit",
        "Delete habit",
        "List habits",
        "Complete habit",
        "Exit"
    ];

    let mut exit = false;
    while !exit {

        println!("Selectd option: ");

        let option_selected = Select::new("Selecione seu cargo:", list.clone())
        .prompt()
        .unwrap();
        
        refresh_status(&mut habits_list);

        match option_selected {
            "Create new habit" => create_new_habit(&mut habits_list, path_habit).await,
            "Edit habit" => edit_habit(path_habit, &mut habits_list).await,
            "Delete habit" => del_habit(path_habit, &mut habits_list).await,
            "List habits" => list_habit(&mut habits_list).await,
            "Complete habit" => complet_habit(path_habit, &mut habits_list, &mut habits_complet_history_list).await,
            "Exit" => exit = true,
            _ => println!("Error, option not found"),
        }
    }
    
}

fn to_read_line (label: &str) -> String {
    let input = Text::new(label)
    .with_default("")
    .prompt()
    .unwrap();
    
    return input.trim().to_string();
}

async fn create_new_habit (habits_list: &mut Vec<Habit>, path: &str) {
    
    println!("Name: ");

    let name: String = to_read_line("Enter the habit name:")
    .parse()
    .expect("Please enter a valid name");

    let time_limit = DateSelect::new("Enter the habit time limit:")
    .prompt()
    .unwrap();

    let habit = Habit {
        id: Uuid::new_v4(),
        name: name,
        status: Status::PENDING,
        create_at: Utc::now(),
        time_limit: time_limit
    };

    let habit_bytes = serde_json::to_vec_pretty(&habit).unwrap();

    let json_text = String::from_utf8(habit_bytes).unwrap();

    println!("{}", json_text);

    habits_list.push(habit);

    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .expect("Failed to open file to save");

    let habit_bytes_list = serde_json::to_vec_pretty(&habits_list).unwrap();

    file.write(&habit_bytes_list).expect("Failed save habit in file");
}

async fn del_habit (path: &str, habit_list: &mut Vec<Habit>) {
    let habit = Select::new("Select habit:", habit_list.iter().map(|h| h.name.to_string()).collect::<Vec<String>>())
    .prompt()
    .unwrap();

    let habit_id: Uuid = habit_list.iter().find(|h| h.name == habit).unwrap().id;

    habit_list.retain(|habit| habit.id != habit_id);

    let update_habit_list_byte = serde_json::to_vec_pretty(&habit_list).unwrap();

    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .expect("Failed to open file to save");

    file.write(&update_habit_list_byte).expect("Failed save new habit list in file");

    println!("Habit deleted");
}

async fn edit_habit (path: &str, habit_list: &mut Vec<Habit>) {

    let habit = Select::new("Select habit:", habit_list.iter().map(|h| h.name.to_string()).collect::<Vec<String>>())
    .prompt()
    .unwrap();

    let habit_id: Uuid = habit_list.iter().find(|h| h.name == habit).unwrap().id;

    if let Some(habit) = habit_list.iter_mut().find(|h| h.id == habit_id) {
        let list = vec![
            "Name",
            "Time limit"
        ];

        let option_selected = Select::new("Select one option", list.clone())
        .prompt()
        .unwrap();

        let mut new_name = || {
            let new_name = to_read_line("New name: ");

            habit.name = new_name
        };

        let mut new_time_limit = || {

            if habit.status == Status::FAILED {
                println!("Habit status is Failed, create new habit");
            }

            let new_time_limit = DateSelect::new("Enter the habit time limit:")
            .prompt()
            .unwrap();

            habit.time_limit = new_time_limit;
        };

        match option_selected {
            "Name" => new_name(),
            "Time limit" => new_time_limit(),
            _ => println!("Error otion not found")
        }

        let habit_byte = serde_json::to_vec_pretty(habit).unwrap();
        
        print!("Habit edited");
        println!("{}", String::from_utf8(habit_byte).unwrap());
    } else {
        println!("This habit not found");
        return;
    }

    let update_habit_byte_list = serde_json::to_vec_pretty(&habit_list).unwrap();

    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .expect("Failed to open file to save");

    file.write(&update_habit_byte_list).expect("Failed save new habit list in file");
}

async fn list_habit (habit_list: &mut Vec<Habit>) {

    for habit in habit_list {
        println!("Name: {} | Status: {:?} | Time Limit: {} | Created At: {:?}", habit.name, habit.status, habit.time_limit, habit.create_at);
    }
}

async fn complet_habit (path: &str, habit_list: &mut Vec<Habit>, complet_history_list: &mut Vec<HabitCompletHistory>) {

    let habit = Select::new("Select habit:", habit_list.iter().map(|h| h.name.to_string()).collect::<Vec<String>>())
    .prompt()
    .unwrap();

    let habit_id: Uuid = habit_list.iter().find(|h| h.name == habit).unwrap().id;

    if let Some(habit) = habit_list.iter_mut().find(|h| h.id == habit_id) {

        let habit_byte = serde_json::to_vec_pretty(habit).unwrap();

        let habit_complet_history = HabitCompletHistory {
            id: Uuid::new_v4(),
            habit_id: habit.id,
            status_alterate: Status::COMPLETED,
            update_at: Utc::now()
        };

        complet_history_list.push(habit_complet_history);

        println!("Habit complet");
        println!("{}", String::from_utf8(habit_byte).unwrap());
    } else {
        println!("This habit not found");
        return;
    }

    let update_habit_byte_list = serde_json::to_vec_pretty(&habit_list).unwrap();
    let update_habit_complet_history_byte_list = serde_json::to_vec_pretty(&complet_history_list).unwrap();

    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .expect("Failed to open file to save");

    let mut file_complet_history = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open("habit_complet_hitory.json")
    .expect("Failed to open file to save");

    file.write(&update_habit_byte_list).expect("Failed save new habit list in file");
    file_complet_history.write(&update_habit_complet_history_byte_list).expect("Failed save new habit complet history list in file");
}