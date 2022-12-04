use std::env;
use std::fs;
use std::str::Split;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let data = fs::read_to_string(file_path)
                .expect("failed reading from file");

    task_1(&data);

    task_2(&data);
}

fn get_doublettes(items_r1: String, items_r2: String) -> Option<String> {
    for item in items_r1.chars() {
        if items_r2.contains(&item.to_string()) {
            return Some(item.to_string());
        }
    }
    return None;
}

fn calc_priority(item: String) -> u32 {
    let ordered_priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    return ordered_priorities.find(item.chars().nth(0).unwrap()).unwrap() as u32 + 1;
}

fn task_1(data: &str) {
    println!("day 3, task 1");
    let mut total_priorities: u32 = 0;
    let rucksacks: Split<&str> = data.split("\n");
    for rucksack in rucksacks {
        let div_position = rucksack.chars().count()/2;
        let items_r1 = &rucksack[..div_position];
        let items_r2 = &rucksack[div_position..];
        match get_doublettes(items_r1.to_string(), items_r2.to_string()) {
            Some(d) => {
                total_priorities = total_priorities + calc_priority(d);    
            },
            None => {}
        }
    }
    println!("total priorities = {}", total_priorities);
}


fn find_common_item(items_r1: String, items_r2: String, items_r3: String) -> Option<String> {
    for item in items_r1.chars() {
        if items_r2.contains(&item.to_string()) {
            if items_r3.contains(&item.to_string()) {
                return Some(item.to_string());
            }
        }
    }
    return None;
}


fn task_2(data: &str) {
    println!("day 3, task 2");
    let mut total_priorities: u32 = 0;
    let teams_count = data.split("\n").count()/3;

    println!("{}", teams_count);

    for i in 0..teams_count {
        let r1 = data.split("\n").nth(i * 3).unwrap();
        let r2 = data.split("\n").nth(i * 3 + 1).unwrap();
        let r3 = data.split("\n").nth(i * 3 + 2).unwrap();
        
        match find_common_item(r1.to_string(), r2.to_string(), r3.to_string()) {
            Some(d) => {
                total_priorities = total_priorities + calc_priority(d);    
            },
            None => {}
        }
    }

    println!("total priorities = {}", total_priorities);
}