use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = &args[1];

    task_1(&data);

    task_2(&data);
}

fn check_valid_packetmarker(token: &str) -> bool {
    let c0 = token.chars().nth(0).unwrap();
    let c1 = token.chars().nth(1).unwrap();
    let c2 = token.chars().nth(2).unwrap();
    let c3 = token.chars().nth(3).unwrap();

    return c0 != c1 && c0 != c2 && c0 != c3 && c1 != c2 && c1 != c3 && c2 != c3;
}

fn task_1(data: &String) {
    println!("day 6, task 1");

    for i in 0..(data.len() - 3) {
        let packetmarker = &data[i..(i + 4)];
        if check_valid_packetmarker(packetmarker) {
            println!("{}", i + 4);
            return;
        }
    }
}

fn check_valid_messagemarker(first: String, marker: String) -> bool {
    if marker.len() == 1 {
        return true;
    }
    if marker.contains(&first) {
        return false;
    } else {
        let first = marker[0..1].to_string();
        let tail = marker[1..].to_string();
        return check_valid_messagemarker(first, tail);
    }
}

fn task_2(data: &String) {
    println!("day 6, task 2");

    for i in 0..(data.len() - 13) {
        let messagemarker = &data[i..(i + 14)].to_string();
        let first = messagemarker[0..1].to_string();
        let tail = messagemarker[1..].to_string();
        if check_valid_messagemarker(first, tail) {
            println!("{}", i + 14);
            return;
        }
    }
}
