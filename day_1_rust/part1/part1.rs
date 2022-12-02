use std::io::stdin;

fn main() {
    let mut calories_per_elf = Vec::new();
    calories_per_elf.push(0);

    loop {
        let mut calorie = String::new();
        let last_elf = calories_per_elf.len();

        println!("Enter next calories for elf {}: ", last_elf);
        stdin().read_line(&mut calorie)
            .expect("Failed to read line");

        calorie = calorie.trim().to_string();
        
        if calorie == "end" { break; }

        if calorie.is_empty() { 
            calories_per_elf.push(0);
            continue;
         }

        match calorie.parse::<i32>() {
            Ok(n) => {
                let last_elf_index =  last_elf - 1;
                let last_elf_calories = calories_per_elf[last_elf_index];
                calories_per_elf[last_elf_index] = last_elf_calories + n;
                println!("Elf {} has {} calories", last_elf , calories_per_elf[last_elf_index]);
            },
            Err(e) => {
                println!("Invalid : {}", e);
                continue;
            }
        };
    }

    let list: &[i32] = &calories_per_elf;
    let mut most_calories_elf = 0;
    let mut most_calories = 0;

    for (i, x) in list.iter().enumerate() {
        if *x > most_calories {
            most_calories = *x;
            most_calories_elf = i + 1;
        }
    }
    
    println!("Most calories: elf {}, amount: {}", most_calories_elf, most_calories);
    
   println!("Exit");
}