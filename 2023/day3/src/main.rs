use std::{
    env,
    fs,
    io::Read,
    vec::Vec,
    collections::HashMap,
};

fn has_symbol(s: &str) -> bool {
    for c in s.chars() {
        if !(c.is_numeric() || c == '.') {
            return true
        }
    }
    false
}

fn has_gear(s: &str) -> usize {
    for (i,c) in s.chars().enumerate() {
        if c == '*' {
            return i
        }
    }
    usize::MAX
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
    let schematic : Vec<String> = contents.lines().map(String::from).collect();

    let mut gear_map = HashMap::<usize,[i32;2]>::new();

    let mut sum: i32 = 0;
    for (i, row) in schematic.iter().enumerate() {
        let row_bytes = row.as_bytes();
        let mut i_lower = i;
        let mut i_upper = i+1;
        if i_lower > 0 {
            i_lower = i_lower - 1;
        }
        if i_upper < schematic.len() {
            i_upper = i_upper + 1;
        }
        let check_rows : &_ = &schematic[i_lower..i_upper];

        let mut j_skip = 0;
        //dbg!(row_bytes);
        for (j, col) in row.chars().enumerate() {
            if j < j_skip {
                continue;
            }

            if col.is_numeric() {
                let mut j_lower = j;
                let mut j_upper = j+1;
                if j_lower > 0 {
                    j_lower = j_lower - 1;
                }
                if j_upper < row.len() {
                    j_upper = j_upper + 1;
                }

                for (crid,check_row) in check_rows.iter().enumerate() {
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
                        let part_id = row.get(k..l).unwrap().parse::<i32>().unwrap();
                        sum += part_id;

                        let mut gear_index = has_gear(&check_row[j_lower..j_upper]);
                        j_skip = l;
                        if gear_index != usize::MAX {
                            gear_index += j_lower;
                            gear_index += (i_lower + crid) * row.len();
                            dbg!(gear_index);
                            match gear_map.get_mut(&gear_index) {
                                Some(value) => {
                                    if value[1] == i32::MAX {
                                        value[1] = part_id;
                                    } else {
                                        value[0] = 0;
                                    }
                                },
                                None => {
                                    gear_map.insert(gear_index, [part_id,i32::MAX]);
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    let mut ratio_sum : i32 = 0;
    for (index, gears) in gear_map {
        if gears[1] != i32::MAX {
            println!("gears: {} {},{}", index, gears[0], gears[1]);
            ratio_sum += gears[0] * gears[1];
        }
    }
    println!("Part 2: {}", ratio_sum);

    println!("Part 1: {}", sum);
}
