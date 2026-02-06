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
