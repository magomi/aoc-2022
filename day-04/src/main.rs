use std::env;
use std::fs;
use std::str::Split;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let data = fs::read_to_string(file_path)
                .expect("failed reading from file");

    task_1(&data);

    task_2(&data);
}

fn task_1(data: &str) {
    println!("day 4, task 1");
    let mut full_overlaps: u32 = 0;
    for assignment_pair in data.split("\n") {
        if !assignment_pair.is_empty() {
            let assignment_1 = assignment_pair.split(",").nth(0).unwrap();
            let assignment_2 = assignment_pair.split(",").nth(1).unwrap();
            let a1_from = assignment_1.split("-").nth(0).unwrap().to_string().parse::<u32>().unwrap();
            let a1_to = assignment_1.split("-").nth(1).unwrap().to_string().parse::<u32>().unwrap();
            let a2_from = assignment_2.split("-").nth(0).unwrap().to_string().parse::<u32>().unwrap();
            let a2_to = assignment_2.split("-").nth(1).unwrap().to_string().parse::<u32>().unwrap();
            
            if a1_from >= a2_from && a1_to <= a2_to || a2_from >= a1_from && a2_to <= a1_to {
                full_overlaps = full_overlaps + 1;
            }
        }
    }
    println!{"full overlaps = {}", full_overlaps};
}

fn task_2(data: &str) {
    println!("day 2, task 1");
    let mut overlaps: u32 = 0;
    for assignment_pair in data.split("\n") {
        if !assignment_pair.is_empty() {
            let assignment_1 = assignment_pair.split(",").nth(0).unwrap();
            let assignment_2 = assignment_pair.split(",").nth(1).unwrap();
            let a1_from = assignment_1.split("-").nth(0).unwrap().to_string().parse::<u32>().unwrap();
            let a1_to = assignment_1.split("-").nth(1).unwrap().to_string().parse::<u32>().unwrap();
            let a2_from = assignment_2.split("-").nth(0).unwrap().to_string().parse::<u32>().unwrap();
            let a2_to = assignment_2.split("-").nth(1).unwrap().to_string().parse::<u32>().unwrap();
            
            if a1_from >= a2_from && a1_from <= a2_to 
            || a1_to >= a2_from && a1_to <= a2_to
            || a2_from >= a1_from && a2_from <= a1_to
            || a2_to >= a1_from && a2_to <= a1_to {
                overlaps = overlaps + 1;
            }
        }
    }
    println!{"overlaps = {}", overlaps};
}