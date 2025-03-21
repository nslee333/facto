use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
use sqlite;

// TODO add cli interface 

fn main() -> io::Result<()> {
    let file_result = File::open("questions.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem with opening file: {error:?}"),
    };

    let reader = io::BufReader::new(file);

    let mut vec = Vec::new();

    
    for line in reader.lines() {
        vec.push(line?);
    }

    Ok(())

}

// fn read_file(file: String) -> Result<Vec<String>, Err> {



// }

fn import_questions_to_db() -> Result<()> {

    let connection = sqlite::open(":memory:").unwrap();

    let query = "
        CREATE TABLE questions (id INTEGER primary key autoincrement, question TEXT);
        INSERT INTO questions VALUES () 
    ";
    // TODO finish up insert, extract this code into library methods

    Ok(())
}