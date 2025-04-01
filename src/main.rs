use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
use rusqlite::{Connection, params};

// TODO add cli interface 

fn main() -> io::Result<()> {

    let file_result = File::open("questions.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem with opening file: {error:?}"),
    };

    let reader = io::BufReader::new(file);

    let mut questions = Vec::new();

    
    for line in reader.lines() {
        questions.push(line?);
    }

    
    let questions_im = questions;
    import_questions_to_db(questions_im);
    Ok(())
}

// fn read_file(file: String) -> Result<Vec<String>, Err> {



// }

fn import_questions_to_db(questions_vector: Vec<String>) -> Result<(), rusqlite::Error> {
    println!("Starting to import questions into the database...");
    let connection_result = Connection::open("questions.db");
    let mut conn = match connection_result {
        Ok(file) => file,
        Err(error) => panic!("Problem getting a connection: {}", error),
    };

    let tx = conn.transaction();
    let tx_res = match tx {
        Ok(ok) => ok,
        Err(error) => panic!("Error occured with creating a transaction: {}", error),
    };

    {
        let mut stmt = tx_res.prepare("INSERT INTO questions (question) VALUES (?) ");
        let stmt_res = match stmt {
            Ok(statement) => statement,
            Err(error) => panic!("Error preparing statement: {}", error),
        };

        for s in &questions_vector {
            stmt?.execute(params![s]);
        }
    }


    let query = "
        CREATE TABLE questions (id INTEGER primary key autoincrement, question TEXT);
        INSERT INTO questions VALUES (question: ) 
    ";

    // TODO finish up insert, extract this code into library methods

    Ok(())
}