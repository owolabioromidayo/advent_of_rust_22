use std::fs;
use std::collections::HashSet;

fn main() {
    let filename = "6.txt";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let vec: Vec<char> = contents.chars().collect();

    // println!("{:?}", vec);
    
    //Part A: start of packet
    for i in 0..contents.len()-4 {
        //look ahead
        let mut local_set : HashSet<char> = HashSet::new(); 

        for j in i..i+4 {
            println!("{} : {} -> {}", i,  j, vec[j]);
        if !local_set.contains(&vec[j]){
                local_set.insert(vec[j]);
            }
            else{
                println!(" Something in set{:?}", local_set);
                break;
            }
        }
        if local_set.len() == 4 {
            println!("{}", i+4); 
            return ;
        }
    }

    //Part B: start of message
    for i in 0..contents.len()-14 {
        //look ahead
        let mut local_set : HashSet<char> = HashSet::new(); 

        for j in i..i+14 {
            println!("{} : {} -> {}", i,  j, vec[j]);
        if !local_set.contains(&vec[j]){
                local_set.insert(vec[j]);
            }
            else{
                println!(" Something in set{:?}", local_set);
                break;
            }
        }
        if local_set.len() == 14 {
            println!("{}", i+14); 
            return ;
        }
    }
}