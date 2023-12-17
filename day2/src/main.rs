use std::fs;

static PATH: &'static str = "./input.txt";

struct Set {
    blue: u32,
    red: u32,
    green: u32,
}

static MAX_SET: Set = Set {
    blue: 14,
    red: 12,
    green: 13
};

impl Set {
    fn is_possible(&self) -> bool {
        return self.blue <= MAX_SET.blue && self.red <= MAX_SET.red && self.green <= MAX_SET.green;
    }

    fn mt() -> Set {
        Set {
            red: 0,
            green: 0,
            blue: 0
        }
    }

    fn parse(tokens: &Vec<&str>) -> Option<Set> {
        // 3 blue, 4 red
        let mut set = Set::mt();
        for i in (0..tokens.len()).step_by(2) {
            let number = tokens[i].parse::<u32>().unwrap();
            let color = tokens[i+1];
            let color = color.replace(",", "");
            match color.as_str() {
                "blue" => set.blue += number,
                "red" => set.red += number,
                "green" => set.green += number,
                _ => return None
            }
        }
        Some(set)
    }
}

fn main() {
    let file = fs::read_to_string(PATH).unwrap();
    let lines = file.lines();
    let mut sum = 0;
    for line in lines {
        println!("{}", line);
        let tokens = line.trim().split(" ").collect::<Vec::<_>>();
        let game_id = tokens[1][..tokens[1].len() - 1].parse::<u32>().unwrap();
        let set_str = &line[line.find(": ").unwrap() + 2..line.len()];
        let sets = set_str.trim().split(";");
        let mut all_valid = true;
        for set in sets {
            let set_tokens = set.trim().split(" ").collect::<Vec::<_>>();
            let set = Set::parse(&set_tokens).unwrap();
            if !set.is_possible() {
                all_valid = false;
            }
        }
        if all_valid {
            sum += game_id;
        }
    }
    println!("sum: {}", sum);
}
