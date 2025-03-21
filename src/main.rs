use std::fs::File;
use std::io::{self, BufRead};



fn main() -> io::Result<()> {
    println!("Hello, world!");
    let file_result = File::open("questions.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem with opening file: {error:?}"),
    };

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
