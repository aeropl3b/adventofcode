use std::{
    env,
    fs,
    vec::Vec,
    io::Read,
};

pub fn read_inputs() -> Option<Vec<String>> {
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
            dbg!(error);
            return None
        },
        Ok(f) => f,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Some(contents.lines().map(String::from).collect())
}
