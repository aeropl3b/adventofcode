use std::{
    env,
    fs,
    io::Read,
    vec::Vec,
    vec,
};

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn parse_string(s: &String) -> i32 {
    let lookup : Vec<String> = vec![
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ].iter().map(|x| x.to_string()).collect::<Vec<String>>();

    let mut lindex : usize = s.len();
    let mut rindex : usize = 0;
    let mut lvalue : i32 = -1;
    let mut rvalue : i32 = -1;
    for (i,m) in lookup.iter().enumerate() {
        match s.find(m) {
            Some(index) => {
                if index < lindex {
                    lindex = index;
                    lvalue = i as i32;
                }
            },
            _ => continue,
        };
        match s.rfind(m) {
            Some(index) => {
                if index >= rindex {
                    rindex = index;
                    rvalue = i as i32;
                }
            },
            _ => continue,
        };
    }
    for (i,c) in s.chars().enumerate() {
        let value : i32 = match (c.to_string()).parse::<i32>() {
            Ok(as_int) => as_int,
            _ =>  continue,
        };

        if i < lindex {
            lindex = i;
            lvalue = value;
        }
        if i >= rindex {
            rindex = i;
            rvalue = value;
        }
    }

    if lindex == rindex {
        rvalue = lvalue;
    }

    dbg!(s);
    dbg!(lvalue);
    dbg!(rvalue);
    assert!(lvalue > 0);
    assert!(rvalue > 0);

    lvalue * 10 + rvalue
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
    let lines : Vec<String> = contents.lines().map(String::from).collect();

    let mut sum : i32 = 0;
    for line in lines {
        sum += parse_string(&line);
    }
    println!("{}", sum);
}
