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

fn task_1(data: &str) {

    println!("day 1, task 1");

    let mut max_sum = 0;

    let elfs: Split<&str> = data.split("\n\n");

    for elf in elfs {
        let items: Split<&str> = elf.split("\n");

        let mut sum: u32 = 0;
        for item in items {
            if !item.is_empty() {
                let item: u32 = item
                    .trim()
                    .parse()
                    .expect("expected a number");
                sum = sum + item;
            }
        }
        if sum > max_sum {
            max_sum = sum;
        }
    }
    println!("max carried by an elf = {}", max_sum); 
}

fn task_2(data: &str) {

    println!("day 1, task 2");

    let mut elfs_sum: HashMap<u32, u32> = HashMap::new();

    let mut elf_cnt: u32 = 0;

    let elfs: Split<&str> = data.split("\n\n");

    for elf in elfs {
        let items: Split<&str> = elf.split("\n");

        let mut sum: u32 = 0;
        for item in items {
            if !item.is_empty() {
                let item: u32 = item
                    .trim()
                    .parse()
                    .expect("expected a number");
                sum = sum + item;
            }
        }
        elfs_sum.insert(elf_cnt, sum);
        elf_cnt += 1;

    }

    let mut elfs_sum: Vec<(&u32, &u32)> = elfs_sum.iter().collect();
    elfs_sum.sort_by(|elf1, elf2| elf1.1.cmp(elf2.1));
    println!("caried by 3 top elfs = {}", elfs_sum[0].1 + elfs_sum[1].1 + elfs_sum[2].1);
}
