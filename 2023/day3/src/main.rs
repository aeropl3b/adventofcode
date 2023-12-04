use std::{
    env,
    fs,
    io::Read,
    vec::Vec,
};

fn has_symbol(s: &str) -> bool {
    for c in s.chars() {
        if !(c.is_numeric() || c == '.') {
            return true
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: {} [filename] ", args[0])
    }

    let filename : String = match args.get(1) {
        None => String::from("aoc.in"),
        Some(name) => name.to_string(),
    };
    let mut file : fs::File = match fs::File::open(filename.clone()) {
        Err(error) => {
            println!("Could not open {}", filename);
            panic!("{}", error)}
        ,
        Ok(f) => f,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut schematic : Vec<String> = contents.lines().map(String::from).collect();

    for (i, row) in schematic.iter().enumerate() {
        let row_bytes = row.as_bytes();
        //dbg!(row_bytes);
        for (j, col) in row.chars().enumerate() {
            if col.is_numeric() {
                let mut i_lower = i;
                let mut i_upper = i;
                let mut j_lower = j;
                let mut j_upper = j;
                if i > 0 {
                    i_lower = i - 1;
                }
                if j > 0 {
                    j_lower = j - 1;
                }
                if i < schematic.len() {
                    i_upper = i + 1;
                }
                if j < row.len() {
                    j_upper = j + 1;
                }

                let check_rows : &_ = &schematic[i_lower..i_upper];
                for check_row in check_rows {
                    if has_symbol(&check_row[j_lower..j_upper]) {
                        let mut k = j;
                        while k > 0 && (row_bytes[k-1] as char).is_numeric() {
                            k -= 1;
                        }
                        let mut l = j;
                        while l < row_bytes.len() && (row_bytes[l] as char).is_numeric() {
                            l += 1;
                        }
                        dbg!(&row.get(k..l));
                    }
                }
            }
        }
    }
}
