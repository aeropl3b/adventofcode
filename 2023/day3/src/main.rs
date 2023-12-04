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

    for (i, row) in schematic.iter().enumerate() {
        for (j, col) in row.char().enumerate() {
            if !(col.is_numeric() || col == '.') {
                // Check adjacent cells
                row[i+1][j+1)]
                row[i-1][j+1)]
                row[i+1][j-1)]
                row[i-1][j-1)]
                row[i][j+1)]
                row[i][j-1)]
                row[i+1][j)]
                row[i-1][j)]
            }
        }
    }
}
