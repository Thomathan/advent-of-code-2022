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

        let split: Vec<&str> = input.split(' ').collect();
        let their_item = get_item(split[0]);
        let our_item = get_item(split[1]);
        
        outcome = get_outcome(our_item,their_item);
        score += outcome + our_item;

        println!("Round score: {}, Total: {}", outcome + our_item, score);
    }
    
    println!("Final score: {}", score);
    println!("Exit");
}

fn get_item(item: &str) -> i32 {
    let value = match item.as_ref() {
        "A" => 1,
        "X" => 1,
        "B" => 2,
        "Y" => 2,
        "C" => 3,
        "Z" => 3,
        _ => {
            println!("Unknown item");
            return 0;
        }
    };
    return value;
}

fn get_outcome(mine: i32, theirs: i32) -> i32  {
    if mine == theirs {
        return 3;
    } 
    else if mine == 3 && theirs == 1 {
        return 0;
    }
    else if (mine == 1 && theirs == 3) || mine > theirs {
        return 6;
    }
    
    return 0;

}