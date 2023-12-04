use std::{
    env,
    fs,
    io::Read,
    vec::Vec,
};

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

    let mut sum: i32 = 0;
    let mut power_sum: i32 = 0;
    let knowns : [i32;3] = [12,13,14];
    for line in lines {
        let mut max : [i32; 3] = [0,0,0];
        dbg!(&line);

        let game_split : Vec<_> = line.split(":").collect();
        // dbg!(&game_split);
        let rounds_split : Vec<_> = game_split[1].trim().split(";").collect();
        // dbg!(&rounds_split);
        for round in rounds_split {
            let counts : Vec<_> = round.trim().split(",").collect();
            for count in counts {
                let color_split : Vec<_> = count.trim().split(" ").collect();
                let ccount = color_split[0].parse::<i32>().unwrap();
                let color = color_split[1].trim().to_lowercase();
                match color.as_str() {
                    "red" => max[0] = std::cmp::max(max[0], ccount),
                    "green" => max[1] = std::cmp::max(max[1], ccount),
                    "blue" => max[2] = std::cmp::max(max[2], ccount),
                    _ => panic!("Unknown Color"),
                };
            }
        }
        dbg!(max);

        if max[0] <= knowns[0] && max[1] <= knowns[1] && max[2] <= knowns[2] {
            let game_no = game_split[0]
                .trim()
                .split(" ")
                .collect::<Vec<_>>()[1]
                .trim()
                .to_string()
                .parse::<i32>().unwrap();
            dbg!(game_no);

            sum += game_no;
        }
        power_sum += max[0] * max[1] * max[2];
    }
    println!("Game Sum: {}", sum);
    println!("Power Sum: {}", power_sum);
}
