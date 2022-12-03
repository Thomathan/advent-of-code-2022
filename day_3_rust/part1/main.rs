use std::io::stdin;

fn main() {
    let mut sum: i32 = 0;

    loop {
        let mut input = String::new();
        let item: i32;

        println!("Enter input:");
        stdin().read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();
        
        if input == "end" { break; }

        // aoc logic
        item = get_common_item_score(input);
        
        sum += item;
        println!("Round sum: {}, item: {}", sum, item );
    }
    
    println!("Final score: {}", sum);
    println!("Exit");
}

fn get_common_item_score(input: String) -> i32 {
    let rank: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let l = input.len();
    let (part1, part2)  = input.split_at(l/2);
    let p1chars: Vec<char> = part1.chars().collect();

    let mut common : char = '0';

    for c in p1chars {
        if part2.contains(c) {
            common = c;
            break;
        }
    }

    let mut index = rank.iter().position(|&r| r == common).unwrap();
    index += 1;

    return index as i32;
}