use uuid::Uuid;
use std::io::Write;
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::fs::{self, OpenOptions, File};
use std::io;

struct OptionsList {
    option_id: u32,
    option_name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Habit {
    id: Uuid,
    name: String,
    status: bool
}

#[tokio::main]
async fn main() {

    let path = "habit.json";

    let mut habits_list: Vec<Habit> = if Path::new(path).exists() {
        let content_file = fs::read_to_string(path).expect("Failed in read file");

        if content_file.trim().is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&content_file).expect("Filed in convert JSON old")
        }
    } else {
        Vec::new()
    };

    if Path::new(path).exists() {
        File::open(path).unwrap()
    } else {
        File::create(path).unwrap()
    };

    let list = vec![
        OptionsList {
            option_id: 1,
            option_name: String::from("create")
        },

        OptionsList {
            option_id: 2,
            option_name: String::from("edit")
        },

        OptionsList {
            option_id: 3,
            option_name: String::from("delet")
        },

        OptionsList {
            option_id: 4,
            option_name: String::from("list")
        },

        OptionsList {
            option_id: 5,
            option_name: String::from("complet")
        },

        OptionsList {
            option_id: 6,
            option_name: String::from("exit")
        }
    ];

    for (indice, _) in list.iter().enumerate() {
        print!("{}", list[indice].option_id);
        println!("-{}", list[indice].option_name);
    }

   

    let mut exit = false;
    while !exit {
       println!("Selectd option: ");

        let option_selected: u32 = to_read_line()
        .parse()
        .expect("Please enter a valid number");

        match option_selected {
            1 => create_new_habit(&mut habits_list, path).await,
            2 => edit_habit(path, &mut habits_list).await,
            3 => del_habit(path, &mut habits_list).await,
            4 => list_habit(&mut habits_list).await,
            5 => complet_habit(path, &mut habits_list).await,
            6 => exit = true,
            _ => println!("Error, option not found"),
        }
    }
    
}

fn to_read_line () -> String {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Error in read_line");
    
    return input.trim().to_string();
}

async fn create_new_habit (habits_list: &mut Vec<Habit>, path: &str) {
    
    println!("Name: ");

    let name: String = to_read_line()
    .parse()
    .expect("Please enter a valid name");

    let habit = Habit {
        id: Uuid::new_v4(),
        name: name,
        status: false
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
    println!("Habit ID: ");

    let habit_id: Uuid = to_read_line().parse().expect("Failed in read input");

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

    println!("Habit ID: ");

    let habit_id: Uuid = to_read_line().parse().expect("Failed in read input");

    if let Some(habit) = habit_list.iter_mut().find(|h| h.id == habit_id) {
        println!("New name: ");

        let new_name = to_read_line().parse().expect("Failed in read input");
        
        habit.name = new_name;

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
    let habit_byte_list = serde_json::to_vec_pretty(&habit_list).unwrap();

    println!("Habits List: ");
    println!("{}", String::from_utf8(habit_byte_list).unwrap())
}

async fn complet_habit (path: &str, habit_list: &mut Vec<Habit>) {
    println!("Habit ID: ");

    let habit_id: Uuid = to_read_line().parse().expect("Failed in read input");

    if let Some(habit) = habit_list.iter_mut().find(|h| h.id == habit_id) {
        habit.status = true;

        let habit_byte = serde_json::to_vec_pretty(habit).unwrap();

        println!("Habit complet");
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