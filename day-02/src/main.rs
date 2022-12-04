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

fn create_lut() -> HashMap<String, u32> {
    /*
    A X rock/1
    B Y paper/2
    C Z scissor/3
    0 lose
    3 draw
    6 win
    */
    let mut lut: HashMap<String, u32> = HashMap::new();
    lut.insert("A X".to_owned(), 4);
    lut.insert("B X".to_owned(), 1);
    lut.insert("C X".to_owned(), 7);
    lut.insert("A Y".to_owned(), 8);
    lut.insert("B Y".to_owned(), 5);
    lut.insert("C Y".to_owned(), 2);
    lut.insert("A Z".to_owned(), 3);
    lut.insert("B Z".to_owned(), 9);
    lut.insert("C Z".to_owned(), 6);
    return lut;
}

fn task_1(data: &str) {
    println!("day 2, task 1");
    let games: Split<&str> = data.split("\n");
    let mut total: u32 = 0;
    let lut = create_lut();
    for game in games {
        if !game.is_empty() {
            total = total + lut.get(game).unwrap();
        }
    }
    println!("total score = {}", total);
}

fn calc_my_move(opp_move: String, goal: String) -> String {
    /* 
    A - rock
    B - paper
    C - scissor

    X - lose
    Y - draw
    Z - win
    */
    if opp_move == "A" && goal == "X" {
        return "C".to_string();
    } else if opp_move == "A" && goal == "Y" {
        return "A".to_string();
    } else if opp_move == "A" && goal == "Z" {
        return "B".to_string();
    } else if opp_move == "B" && goal == "X" {
        return "A".to_string();
    } else if opp_move == "B" && goal == "Y" {
        return "B".to_string();
    } else if opp_move == "B" && goal == "Z" {
        return "C".to_string();
    } else if opp_move == "C" && goal == "X" {
        return "B".to_string();
    } else if opp_move == "C" && goal == "Y" {
        return "C".to_string();
    } else if opp_move == "C" && goal == "Z" {
        return "A".to_string();
    }
    return "X".to_string()  ;
}

fn calc_score(my_move: String, result: String) -> u32 {
    let mut score: u32 = 0;

    if my_move == "A" {
        score = score + 1;
    }
    if my_move == "B" {
        score = score + 2;
    }
    if my_move == "C" {
        score = score + 3;
    }
    if result == "X" {
        score = score + 0;
    }
    if result == "Y" {
        score = score + 3;
    }
    if result == "Z" {
        score = score + 6;
    }

    return score;
}

fn task_2(data: &str) {
    println!("day 2, task 2");
    let games: Split<&str> = data.split("\n");
    let mut total: u32 = 0;
    for game in games {
        if !game.is_empty() {
            let o_move = game.chars().nth(0).unwrap();
            let goal = game.chars().nth(2).unwrap();
            let m_move = calc_my_move(o_move.to_string(), goal.to_string());
            
            let score = calc_score(m_move.to_string(), goal.to_string());
            total = total + score;
        }
    }
    println!("total score = {}", total);
}