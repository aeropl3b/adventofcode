#![feature(extract_if)]
use aoc_tools::cli;

fn main() {
    let input = cli::read_inputs().unwrap();

    dbg!(&input);

    let mut sum : u32 = 0;
    let mut total_cards : u32 = input.len().try_into().unwrap();
    let mut copies : Vec<u32> = Vec::new();
    copies.resize_with(input.len(), Default::default);

    for (index,card) in input.iter().enumerate() {
        let contents : &str = card.split(":").collect::<Vec<_>>()[1].trim();
        let numbers_strs : Vec<_> = contents.split("|").collect();
        let mut winning_numbers : Vec<i32> = vec![];

        for num in numbers_strs[0].trim().split(" ").collect::<Vec<_>>() {
            match num.trim().parse::<i32>() {
                Ok(n) => winning_numbers.push(n),
                _ => continue
            };
        }

        dbg!(&winning_numbers);
        let mut matches : u32 = 0;
        let mut value : u32 = 1;
        for num in numbers_strs[1].trim().split(" ").collect::<Vec<_>>() {
            let check : i32 = match num.trim().parse::<i32>() {
                Ok(n) => n,
                _ => continue
            };

            if winning_numbers.contains(&check) {
                matches += 1;
                value = value << 1;
                dbg!(&check);
            }
        }
        dbg!(&matches);
        if matches > 0 {
            let from_index = index as u32 + 1;
            let to_index = from_index + matches;
            dbg!(from_index..to_index);
            for i in from_index..to_index {
                copies[i as usize] += copies[index] + 1;
            }
        }

        total_cards += copies[index];
        sum = sum + (value>>1);
    }
    dbg!(&copies);
    println!("Sum of values: {}", sum);
    println!("Total Cards: {}", total_cards);
}
