use std::fs;

fn main() {
    let filename = "1.txt";
    let empty = "";
    let mut max = 0;
    let mut sum = 0;
    let mut curr = 0;
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let vec: Vec<&str> = contents.split("\n").collect();

    for i in &vec{
        if i.eq(&empty) {
            sum = 0;
        } else {
            curr = i.trim().parse().expect("Want a number.");
            sum += curr;
            if sum > max{
                max = sum;
                sum = 0;
            }
        }
    }

    println!("Max is {} \n", max);
}