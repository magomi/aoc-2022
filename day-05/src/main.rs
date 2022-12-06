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

struct Move {
    count: u32,
    source_stack: u32,
    target_stack: u32,
}

fn parse_to_stacks(data: &str) -> Vec<Vec<char>>{
    let mut stacks = Vec::new();

    let mut stack_levels = data.split("\n").collect::<Vec<&str>>();

    let stacks_count = stack_levels[stack_levels.len() - 1].split("   ").count();
    stack_levels = stack_levels[0..(stack_levels.len() - 1)].to_vec();
    let mut reversed_stack_levels = Vec::new();
    let stack_levels_count = stack_levels.len();
    for i in 0..stack_levels_count {
        reversed_stack_levels.push(stack_levels[stack_levels_count - i - 1]);
    }

    for i in 0..stacks_count {
        stacks.push(Vec::new());
    }
    
    for level in reversed_stack_levels {
        for i in 0..stacks_count {
            match level.chars().nth(1 + i * 4) {
                Some(c) => {
                    if c != ' ' {
                        stacks[i].push(c);
                    }
                }
                None => {}
            }
        }
    }
    
    return stacks;
}

fn parse_moves(data: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    for move_data in data.split("\n") {
        println!("{}", move_data);
        let move_items: Vec<&str> = move_data.split(" ").collect();
        moves.push(Move {
            count: move_items[1].parse().unwrap(), 
            source_stack: move_items[3].parse().unwrap(), 
            target_stack: move_items[5].parse().unwrap()});
    }
    return moves;
}

fn task_1(data: &str) {
    println!("day 5, task 1");
    let initial_stacks_data = data.split("\n\n").nth(0).unwrap();
    let moves_data = data.split("\n\n").nth(1).unwrap();
    let mut stacks = parse_to_stacks(initial_stacks_data);
    let moves = parse_moves(moves_data);
 
    for mv in moves {
        for i in 0..mv.count {
            let item = stacks[(mv.source_stack - 1) as usize].pop();
            stacks[(mv.target_stack - 1) as usize].push(item.unwrap());
        }
    }

    for stack in stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!("");
}


fn task_2(data: &str) {
    println!("day 5, task 2");
    let initial_stacks_data = data.split("\n\n").nth(0).unwrap();
    let moves_data = data.split("\n\n").nth(1).unwrap();
    let mut stacks = parse_to_stacks(initial_stacks_data);
    let moves = parse_moves(moves_data);
 
    for mv in moves {
        let mut tmp = Vec::new();
        for i in 0..mv.count {
            let item = stacks[(mv.source_stack - 1) as usize].pop();
            tmp.push(item.unwrap());
        }
        for i in 0..mv.count {
            let item = tmp.pop();
            stacks[(mv.target_stack - 1) as usize].push(item.unwrap());
        }
    }

    for stack in stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!("");

}