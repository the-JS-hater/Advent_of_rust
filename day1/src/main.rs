use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};

fn main() {
    calc_fuel();
    alt_calc_fuel();
}


fn calc_fuel() -> io::Result<()>{
    let file = match File::open("day1.txt") {
        Ok(f) => f,
        Err(e) => return Err(e), // Return early with an error if file opening fails
    };

    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines(){
        let num = line.unwrap().trim().parse::<i32>().unwrap(); 
        sum += num / 3 - 2 //integer division in Rust discards fractionals and thus naturally
                           //rounds down
    }
    println!("{}", sum);
    return Ok(());
}



fn alt_calc_fuel() -> io::Result<()>{
    let file = match File::open("day1.txt") {
        Ok(f) => f,
        Err(e) => return Err(e), // Return early with an error if file opening fails
    };

    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines(){
        let mut num = line.unwrap().trim().parse::<i32>().unwrap(); 
        loop {
            num = num / 3 - 2; //integer division in Rust discards fractionals and thus naturally
            if num <= 0{
                break;
            }
            sum += num;
        }
    }
    println!("{}", sum);
    return Ok(());
}




