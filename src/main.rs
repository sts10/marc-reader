use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let raw_records = make_raw_records("./test-data/test_10.mrc");
    println!("Found {} raw_records.", raw_records.len());
    for raw_record in raw_records {
        let parsed_record: Record = parse_raw_record(raw_record.to_vec());
        println!(
            "Leader: {}",
            parsed_record.leader.iter().collect::<String>()
        );
        for field in parsed_record.fields {
            println!("{} : {}", field.tag, field.value);
        }
    }
}

// The Directory immediately follows the Leader at the beginning
// of the record and is located at character position 24.
// Each Directory entry is 12 character positions in length and
// contains three portions: the field tag, the field length,
// and the starting character position.

// 00-02 - Tag
// Three ASCII numeric or ASCII alphabetic characters (upper case or lower case, but not both) that identify an associated variable field.
// 03-06 - Field length
// Four ASCII numeric characters that specify the length of the variable field, including indicators, subfield codes, data, and the field terminator. A Field length number of less than four digits is right justified and unused positions contain zeros.
// 07-11 - Starting character position
// Five ASCII numeric characters that specify the starting character position of the variable field relative to the Base address of data (Leader/12-16) of the record. A Starting character position number of less than five digits is right justified and unused positions contain zeros.

#[derive(Debug, Clone)]
struct Record {
    data: Vec<char>,
    fields: Vec<Field>,
    leader: Vec<char>, // It'd be neat to have this set to 24 characters?  &'a [char; 24],
}

#[derive(Debug, Clone)]
struct Field {
    tag: String,      // both Control and Data fields
    value: String,    // for Control fields
    indicator1: char, // for Data fields. Maybe single char?
    indicator2: char, // for Data fields. Maybe single char?
                      // sub_fields: &'a [char], // for Data fields. Eventually we'll use a SubField struct as the type here.
}

// #[derive(Debug)]
// struct SubField<'a> {
//     code: &'a [char],  // e.g. "a"
//     value: &'a [char], // e.g. "Diabetes"
// }

fn parse_raw_record(raw_record: Vec<char>) -> Record {
    let mut fields: Vec<Field> = vec![];

    // this is awfu but it's the most sure way of finding the character langth of
    // the raw directory, which we need to know the starting position offset!
    let mut directory_size = 0;
    for ch in &raw_record[24..raw_record.len()] {
        if *ch == 0x1e as char {
            break;
        } else {
            directory_size = directory_size + 1;
        }
    }

    let starting_character_position_offset = directory_size + 24;
    let leader: &Vec<char> = &raw_record[0..24].to_vec(); // inefficient?

    for raw_directory_entry in raw_record[24..raw_record.len()].chunks_exact(12) {
        if raw_directory_entry.contains(&(0x1e as char)) {
            break;
        }
        let field_length: usize = number_cleaner(&raw_directory_entry[3..=6]);
        let starting_character_position: usize = number_cleaner(&raw_directory_entry[7..=11]);
        let starting_character_position =
            starting_character_position + starting_character_position_offset;
        // Let's make this easier and use a String
        let value: String = raw_record
            [starting_character_position..starting_character_position + field_length]
            .iter()
            .collect();

        // Best guess as to where these indicators are...
        let indicator1 = &value
            .chars()
            .nth(0)
            .expect("Value too short to have an first indicator");
        let indicator2 = &value
            .chars()
            .nth(1)
            .expect("Value too short to have an second indicator");

        let this_field = Field {
            tag: raw_directory_entry[0..=2].iter().collect(),
            value, // for Control fields
            indicator1: *indicator1,
            indicator2: *indicator2,
            // sub_fields: &'a [char], // for Data fields. Eventually we'll use a SubField struct as the type here.
        };
        fields.push(this_field);
    }

    let this_record: Record = Record {
        leader: leader.to_vec(),
        data: raw_record.clone(), // I'm tired!
        fields: fields.clone(),
    };
    this_record
}

fn _print_record_type(record: &[char]) {
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

/// ```
/// assert_eq!(number_cleaner2(&['0', '0', '1', '1']), 11);
/// assert_eq!(number_cleaner2(&['0', '0', '0', '5', '4']), 54);
/// assert_eq!(number_cleaner2(&['0', '0', '1', '0', '7']), 107);
/// ```
fn number_cleaner(chs: &[char]) -> usize {
    let as_string: String = chs.iter().collect();
    as_string.parse().unwrap()
}
