use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "3.txt";
    let mut sum = 0;
    let mut matches: Vec<char> = Vec::new();

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let vec: Vec<&str> = contents.split("\n").collect();

    for s in &vec{
        let length = s.len();
        let mid = s.len() / 2;
        let l_slice = &s[..mid];
        let r_slice = &s[mid..];

        let mut local_set: HashSet<char> = HashSet::new();

        println!("{} : {}", l_slice, r_slice);
        for l in l_slice.chars(){
            if r_slice.contains(l) {
                local_set.insert(l);
                // println!("{} {} {}", l_slice, r_slice, &l);
                // println!("{} : {} -> {l}", l_slice, r_slice);
                println!("{l}");
            }
        }

        for ch in &local_set{
            matches.push(*ch);
        }
    }

    
    for l in &matches{
        let ascii_val: u32 = *l as u32;
        let mut calc: u32 = 0;
        if l.is_lowercase(){
            calc = ascii_val  - 96 ;
        } else{
            calc = ascii_val  - 38 ;
        }
        sum += calc;
    }


    println!("Sum is {}", sum);
    //for three grouping ( Part B)

    let mut badges: Vec<char> = Vec::new();
    for i in 0..vec.len() {
        if (i) % 3 == 0{

            println!("{}", vec[i]);
            println!("{}", vec[i+1]);
            println!("{}", vec[i+2]);

            for l in vec[i].chars() {
                if vec[i+1].contains(l) && vec[i+2].contains(l){
                    badges.push(l);
                    println!("{l}\n");
                    break;
                }
            }

        } else{
            continue;
        }

    }
    
    let mut bsum: u32 = 0;
    for l in &badges{
        let ascii_val: u32 = *l as u32;
        let mut calc: u32 = 0;
        if l.is_lowercase(){
            calc = ascii_val  - 96 ;
        } else{
            calc = ascii_val  - 38 ;
        }
        bsum += calc;
    }
    println!("Badge sum is {}", bsum);
}
