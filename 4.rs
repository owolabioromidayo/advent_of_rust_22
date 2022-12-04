use std::fs;

fn main() {
    let filename = "4.txt";
    let mut _count = 0;
    let mut _count2 = 0;

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let vec: Vec<&str> = contents
                        .trim()
                        .split("\n")
                        .collect();


    //Part A  (check if one range contains another entirely)
    //Part B(check if there is any overlap at all)
    for s in &vec{
        let nums: Vec<i32> = s.split(|c| c == '-' || c == ',')
                                    .map(|s| s.parse().unwrap())
                                    .collect();
        let mut ct = 0; 

        println!("{:?}", nums);
        
        let r1 = nums[0]..nums[1]+1;
        let r2 = nums[2]..nums[3]+1;

        let r1_len = r1.len();
        let r2_len = r2.len();

        if r1_len >= r2_len{
            //check if r2 inside r1
            for n in r2{
                if r1.contains(&n){
                    ct += 1;
                }
            }
            if ct == r2_len{
                _count += 1 
            }
        } else{
            //check if r1 inside r2
            for n in r1{
                if r2.contains(&n){
                    ct += 1;
                }
            }
            if ct == r1_len{
                _count += 1;
                println!("Found one");
            }
        }
        
        if ct > 0{
            _count2 += 1;
            println!("Found one Part B");
        }
        
    println!("");
    }
    println!("Final count: {}", _count);
    println!("Final count Part B: {}", _count2);
}
