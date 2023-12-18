use std::fs;
static PATH: &'static str = "./input.txt";

struct Card {
    winning_numbers: Vec<u64>,
    our_numbers: Vec<u64>
}

impl Card {
    fn parse(line: &str) -> Result<Card, ()> {
        let numbers = &line[line.find(":").unwrap()+1..line.len()];
        let split: Vec<_> = numbers.split("|").collect();
        let get_numbers = |s: &str| -> Vec<u64> { 
            s
                .trim().split(" ")
                .filter(|s| !s.trim().is_empty())
                .map(|s: &str| -> u64 { s.parse::<u64>().unwrap() }).collect() 
        };
        Ok(Card {
            winning_numbers: get_numbers(split[0]),
            our_numbers: get_numbers(split[1])
        })
    }
    
    fn score(&self) -> u64 {
        let mut pow: i32 = -1;
        for num in &self.our_numbers {
            if self.winning_numbers.contains(num) {
                pow += 1;
            }
        }
        match pow {
            -1 => 0,
            _ => 1 << pow
        }
    }
}

fn main() {
    let file = fs::read_to_string(PATH).unwrap();
    let lines = file.lines();
    let mut sum = 0;

    for line in lines {
        let card = Card::parse(line).unwrap();
        sum += card.score();
    }

    println!("SUM: {}", sum);
}
