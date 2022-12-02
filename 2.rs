use std::fs;

fn main() {
    let filename = "2.txt";
    let mut sum = 0;

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let vec: Vec<&str> = contents.split("\n").collect();

    for i in &vec{
        match i.trim() as &str{
            // "A X" => { sum += 4;},
            // "A Y" => {sum += 8;},
            // "A Z" => {sum += 3;} ,
            // "B X" => {sum += 1;},
            // "B Y" => {sum += 5;},
            // "B Z" => {sum += 9;},
            // "C X" => {sum += 7;},
            // "C Y" => {sum += 2;},
            // "C Z" => {sum += 6;},
            "A X" => { sum += 3;},
            "A Y" => {sum += 4;},
            "A Z" => {sum += 8;} ,
            "B X" => {sum += 1;},
            "B Y" => {sum += 5;},
            "B Z" => {sum += 9;},
            "C X" => {sum += 2;},
            "C Y" => {sum += 6;},
            "C Z" => {sum += 7;},
            _ => { sum += 0;},
        }
    }

    println!("Sum is {} \n", sum);
}