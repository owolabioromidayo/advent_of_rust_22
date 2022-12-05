
use std::fs;

fn main() {
    let filename = "5.txt";
    let mut _count = 0;
    let mut _count2 = 0;

    let mut crates: Vec<Vec<char>> = vec![vec!['B', 'L', 'D', 'T', 'W', 'C', 'F', 'M'], 
                                            vec!['N', 'B', 'L'], 
                                            vec!['J', 'C', 'H','T','L','V'],
                                            vec!['S','P','J','W'],
                                            vec!['Z','S','C','F','T','L','R'],
                                            vec!['W','D','G','B','H','N','Z'],
                                            vec!['F', 'M','S','P','V','G','C','N'],
                                            vec!['W','Q','R','J','F','V','C','Z'],
                                            vec!['R','P','M','L','H'],
                                            ];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let vec: Vec<&str> = contents
                        .trim()
                        .split("\n")
                        .collect();


    //Part A (find what is on top of the vectors) 
    let mut instructions : Vec<Vec<usize>>  = Vec::new();

    //extract the numbers from the instructions
    for line in &vec{
        let s: Vec<&str> = line.trim().split(' ').collect();

        let i1: usize = s[1].parse().unwrap();
        let i2: usize = s[3].parse().unwrap();
        let i3: usize = s[5].parse().unwrap();

        instructions.push(vec![i1,i2,i3]);

    }

    for instruction in &instructions{
        let _count = instruction[0];
        let from: usize = instruction[1];
        let to: usize = instruction[2];

        let mut temp: char = 'O';
        let start = crates[from-1].len() - _count;

        // for _i in 0.._count {

        //     temp = crates[from-1].pop().unwrap(); 
        //     crates[to-1].push(temp);
        // }

        //PartB : move multiple at once, retaining order

            //push in batch
        for _i in  start..crates[from-1].len() {
            temp = crates[from-1][_i];
            crates[to-1].push(temp);
        }
        //remove in batch
        for _i in  start..crates[from-1].len() {
            crates[from-1].pop();
        }
    }

    for section in &crates{
        print!("{}", section[section.len()-1]);
    }
    println!("");


}
