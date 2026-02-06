use std::fs::File;
use std::io;
use std::io::prelude::*;
fn main() {
    let records = make_raw_records("./test-data/test_10.mrc");
    println!("Found {} records.", records.len());
    // 3. Figure out how to parse the LEADER of each of these records
    for record in records {
        print_record_type(&record);
        // This is a bald attempt at learning about fields, but I
        // I think I need to handle the Leader and the Directory
        // first.
        let mut field_count = 0;
        let mut fields: Vec<Vec<char>> = vec![vec![]]; // not sure if this initalization is corrent.
        for ch in record {
            fields[field_count].push(ch);
            if ch == 0x1e as char {
                // println!("found end of a field");
                field_count = field_count + 1;
                fields.push(vec![]);
            }
        }
        println!("This record has {} fields", field_count);
        // 4. Parse field 245 (title) of each Record
        for field in fields {
            // Nope...
            if field.len() > 5 && field[0..2] == ['2', '4', '5'] {
                println!("{:?}", field);
            }
        }
    }
}

fn _take_until(input: Vec<char>, delimiter: char) -> Option<Vec<char>> {
    let mut output: Vec<char> = vec![];
    for ch in input {
        if ch == delimiter {
            return Some(output);
        } else {
            output.push(ch);
        }
    }
    None
}

fn print_record_type(record: &[char]) {
    let leader: &[char; 24] = record[0..24].try_into().unwrap();
    let _record_length = &leader[0..5];
    let record_type = match leader[6] {
        'a' => "Language material",
        'c' => "Notated music",
        'd' => "Manuscript notated music",
        'e' => "Cartographic material",
        'f' => "Manuscript cartographic material",
        'g' => "Projected medium",
        'i' => "Nonmusical sound recording",
        'j' => "Musical sound recording",
        'k' => "Two-dimensional nonprojectable graphic",
        'm' => "Computer file",
        'o' => "Kit",
        'p' => "Mixed materials",
        'r' => "Three-dimensional artifact or naturally occurring object",
        't' => "Manuscript language material ",
        _ => "Unknown type",
    };
    // println!("Record length is {:?}", record_length);
    println!("Record type is {}", record_type);
}

/// Delimiting on `0x1d as char`, chunk out individual records
/// as Vector of characters
fn make_raw_records(file_name: &str) -> Vec<Vec<char>> {
    // let chars = read_string_from_file_to_vector("./my-data/test_10.mrc").unwrap();
    let chars = read_string_from_file_to_vector(file_name).unwrap();
    let mut records: Vec<Vec<char>> = vec![vec![]]; // not sure if this initalization is corrent.
    let mut idx = 0;
    for ch in chars {
        records[idx].push(ch);
        if ch == 0x1d as char {
            // println!("Found end of a record!");
            idx = idx + 1;
            records.push(vec![]);
        }
    }
    // my poor code requires us to trim off the last record, which is
    // an empty Vector
    records.pop();
    records
}

/// Reads a text file into a Vector of `char`s (characters)
pub fn read_string_from_file_to_vector(file_path: &str) -> io::Result<Vec<char>> {
    let mut f = File::open(file_path.trim_matches(|c| c == '\'' || c == ' '))?;
    let mut string_from_file = String::new();
    f.read_to_string(&mut string_from_file)
        .expect("something went wrong reading the file");

    // println!("String: {}", string_from_file);
    let mut vector_of_chars = Vec::new();
    for c in string_from_file.chars() {
        // print!("{}", c);
        vector_of_chars.push(c);
    }
    Ok(vector_of_chars)
}
