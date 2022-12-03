use std::io::stdin;

fn main() {
    let mut sum: i32 = 0;
    let mut current_line = 0;
    let mut lines: [String; 3] = Default::default();

    loop {
        if current_line == 3 {
            current_line = 0;
        }
        let mut input = String::new();
        let item: i32;

        println!("Enter input for {}: ", current_line);
        stdin().read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();
        
        if input == "end" { break; }

        lines[current_line] = input.clone();

        if current_line < 2 {
            lines[current_line] = input;
            current_line += 1;
            continue;
        } else if current_line == 2 {
            lines[current_line] = input;
            current_line = 0;
        }

        // aoc logic
        item = get_common_item_score(lines.clone());
        
        sum += item;
        println!("Round sum: {}, item: {}", sum, item );
    }
    
    println!("Final score: {}", sum);
    println!("Exit");
}

fn get_common_item_score(input: [String; 3]) -> i32 {
    let rank: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let line1: Vec<char> = input[0].chars().collect();

    let line2 = &input[1];
    let line3  = &input[2];

    let mut common : char = '0';
    let mut common_chars = Vec::new();

    for c in line1 {
        if line2.contains(c) {
            common_chars.push(c);
        }
    }
    for d in common_chars {
        if line3.contains(d) {
            common = d;
            break;
        }
    }

    let mut index = rank.iter().position(|&r| r == common).unwrap();
    index += 1;

    return index as i32;
}