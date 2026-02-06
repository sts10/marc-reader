use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let records = make_raw_records("./test-data/test_10.mrc");
    println!("Found {} records.", records.len());
    // 3. Figure out how to parse the LEADER of each of these records
    for record in records {
        print_record_type(&record);
        let directory = get_directory(&record);
        let mut directory_as_structs = HashMap::new(); // <&[char, Entry>
        for d_entry in directory {
            let parsed_d_entry = parse_single_directory_entry(d_entry);
            directory_as_structs.insert(
                parsed_d_entry.tag.iter().collect::<String>(),
                parsed_d_entry,
            );
        }
        // Now we need to use the information isn each directory
        // entry to go look-up information?

        // We'll start with field 245
        println!("{:?}", directory_as_structs["245"]);
        let title_field_length = number_cleaner(directory_as_structs["245"].field_length);
        let title_starting_character_position =
            number_cleaner(directory_as_structs["245"].starting_character_position);
        // Friday TO DO:
        // Figure out where the starting_character_position actually
        // starts from...
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

// The Directory immediately follows the Leader at the beginning
// of the record and is located at character position 24.
// Each Directory entry is 12 character positions in length and
// contains three portions: the field tag, the field length,
// and the starting character position.
//
// How do I find the end of the directory?
// One way may be to take until the first 0x1e we find
//
// The output type of this function should probably be Vec<&[char; 12]>
// But I'm taking the easy way for now
fn get_directory(record: &[char]) -> Vec<&[char]> {
    // let record_minus_leader: Vec<char> = record[24..record.len()].try_into().unwrap();
    let mut directory: Vec<&[char]> = vec![];
    // We know each Directory entry is exactly 12 characters
    for entry in record[24..record.len()].chunks_exact(12) {
        // 0x1e is marks the end of the directory
        if entry.contains(&(0x1e as char)) {
            return directory;
        }
        directory.push(entry);
    }
    panic!("Error parsing a record's Directory");
}

/// A single directory entry (12 characters)
#[derive(Debug)]
struct DEntry<'a> {
    tag: &'a [char],                         // lngth of 3
    field_length: &'a [char],                // length of 4
    starting_character_position: &'a [char], // length of 5
}
// 00-02 - Tag
// Three ASCII numeric or ASCII alphabetic characters (upper case or lower case, but not both) that identify an associated variable field.
// 03-06 - Field length
// Four ASCII numeric characters that specify the length of the variable field, including indicators, subfield codes, data, and the field terminator. A Field length number of less than four digits is right justified and unused positions contain zeros.
// 07-11 - Starting character position
// Five ASCII numeric characters that specify the starting character position of the variable field relative to the Base address of data (Leader/12-16) of the record. A Starting character position number of less than five digits is right justified and unused positions contain zeros.
fn parse_single_directory_entry(d_entry: &[char]) -> DEntry<'_> {
    let tag: &[char] = &d_entry[0..=2];
    let field_length: &[char] = &d_entry[3..=6];
    let starting_character_position: &[char] = &d_entry[7..=11];

    DEntry {
        tag: tag,
        field_length: field_length,
        starting_character_position: starting_character_position,
    }
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

/// ```
/// assert_eq!(number_cleaner2(&['0', '0', '1', '1']), 11);
/// assert_eq!(number_cleaner2(&['0', '0', '0', '5', '4']), 54);
/// assert_eq!(number_cleaner2(&['0', '0', '1', '0', '7']), 107);
/// ```
fn number_cleaner(chs: &[char]) -> usize {
    let as_string: String = chs.iter().collect();
    as_string.parse().unwrap()
}
