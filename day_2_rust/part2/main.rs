use std::io::stdin;

fn main() {
    let mut score: i32 = 0;

    loop {
        let mut input = String::new();
        let outcome: i32;

        println!("Enter input:");
        stdin().read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();
        
        if input == "end" { break; }

        let my_item = get_item(input.as_ref());

        let split: Vec<&str> = input.split(' ').collect();
        outcome = get_outcome(split[1]);

        score += outcome + my_item;

        println!("Round score: {}, Total: {}", outcome + my_item, score);
    }
    
    println!("Final score: {}", score);
    println!("Exit");
}

fn get_item(item: &str) -> i32 {
    let value = match item.as_ref() {
        "A X" => 3,
        "A Y" => 1,
        "A Z" => 2,
        "B X" => 1,
        "B Y" => 2,
        "B Z" => 3,
        "C X" => 2,
        "C Y" => 3,
        "C Z" => 1,
        _ => {
            println!("Unknown item");
            return 0;
        }
    };
    return value;
}

fn get_outcome(item: &str) -> i32  {
    let value = match item.as_ref() {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => {
            println!("Unknown item");
            return 0;
        }
    };
    return value;

}