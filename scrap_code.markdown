This one might actually be useful in the future?
```rust
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

/// A single directory entry (12 characters)
#[derive(Debug)]
struct DEntry<'a> {
    tag: &'a [char],                         // lngth of 3
    field_length: &'a [char],                // length of 4
    starting_character_position: &'a [char], // length of 5
}
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
```


```rust

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
            // let string_to_print: String = if field.len() > 72 {
            //     field[0..70].iter().collect()
            // } else {
            //     field.iter().collect()
            // };
            let _string_to_print: String = field.iter().collect();
        }

```

More garbage
```rust

        // Now we need to use the information isn each directory
        // entry to go look-up information?

        //         // We'll start with field 245
        //         let title_field_length = number_cleaner(directory_as_structs["245"].field_length);
        //         let title_starting_character_position =
        //             number_cleaner(directory_as_structs["245"].starting_character_position);

        //         // Adjust for offset of directory and leader characters
        //         let title_starting_character_position =
        //             title_starting_character_position + raw_directory.len() * 12 + 24;

        //         let actual_title = &raw_record[title_starting_character_position
        //             ..title_starting_character_position + title_field_length];
        //         // I'm not sure about the first few characters of the actual title
        //         let indicator1 = actual_title[0];
        //         let indicator2 = actual_title[1];
        //         println!("{}", actual_title[2..].iter().collect::<String>());
```

```rust

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

fn make_directory_of_record(record: &[char]) -> HashMap<String, DEntry<'_>> {
    let directory = get_directory(&record);
    let mut directory_as_structs = HashMap::new(); // <&[char, DEntry>
    for d_entry in &directory {
        let parsed_d_entry = parse_single_directory_entry(d_entry);
        directory_as_structs.insert(
            parsed_d_entry.tag.iter().collect::<String>(),
            parsed_d_entry,
        );
    }
    directory_as_structs
}
```

