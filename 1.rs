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

    let mut sums: Vec<i32> = Vec::new();

    for i in &vec{
        if i.eq(&empty) {
            if sum > max{
                max = sum;
            }
            sums.push(sum);
            sum = 0;
        } else {
            curr = i.trim().parse().expect("Want a number.");
            sum += curr;
        }
    }

    println!("Max is {} \n", max);

    sums.sort();
    //Part B (Get sum of max 3) 
    let sum2: i32 = sums[sums.len()-3..sums.len()].iter().sum();
    println!("{:?} \n", &sums[sums.len()-3..sums.len()]);
    println!("3Sum is {} \n", sum2);

}