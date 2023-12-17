use std::fs;
static PATH: &'static str = "./input.txt";

type Schematic<'a> = Vec<Vec<char>>;


#[derive(PartialEq, Debug)]
struct SchematicNumber {
    row: usize,
    col_start: usize,
    length: usize,
}

fn is_symbol(c: char) -> bool {
    match c {
        '0'..='9' => false,
        ' ' => false,
        '.' => false,
        _ => true
    }
}

impl SchematicNumber {
    fn parse(map: &Schematic, last_parsed_index: usize) -> Option<SchematicNumber> {
        let start_row = last_parsed_index/map[0].len();
        let start_row_col = last_parsed_index % map[0].len();
        let mut number = SchematicNumber::default();
        for row in start_row..map.len() {
            for col in 0..map[0].len() {
                if start_row == row && col < start_row_col { 
                    continue;
                }

                let chr = map[row][col];
                if chr.is_numeric() {
                    if number.length == 0 {
                        // if length == 0, then we just started run
                        number.row = row;
                        number.col_start = col;
                    }
                    number.length += 1
                } else if number.length != 0 {
                    return Some(number);
                }
            }
            if number != SchematicNumber::default() {
                return Some(number);
            }  
        }
        None
    }

    fn is_valid(&self, map: &Schematic) -> bool {
        let col_start = if self.col_start > 0 {self.col_start - 1} else {self.col_start};
        let col_end = std::cmp::min(self.col_start+self.length, map[0].len()-1);
        let row_start = if self.row > 0 {self.row - 1} else {self.row};
        let row_end = std::cmp::min(map.len()-1, self.row+1);
        for col in col_start..=col_end {
            for row in row_start..=row_end {
                let chr = map[row][col]; 
                if is_symbol(chr) {
                    return true
                }
            }
        }
        false
    }

    fn as_int(&self, map: &Schematic) -> u64 {
        let str: String = map[self.row][self.col_start..self.col_start+self.length].iter().collect();
        str.parse::<u64>().unwrap()
    }

    fn default() -> SchematicNumber {
        SchematicNumber {
            row: 0,
            col_start: 0,
            length: 0
        }
    }
}

fn main() {
    let file = fs::read_to_string(PATH).unwrap();
    let lines = file.lines();
    let mut map: Schematic = Vec::new();
    for line in lines {
        map.push(line.chars().collect());
    }
    let mut offset: usize = 0;
    let mut sum: u64 = 0;
    while let Some(number) = SchematicNumber::parse(&map, offset) {
        offset = number.row * map[0].len() + number.col_start + number.length;
        if number.is_valid(&map) {
            let num = number.as_int(&map);
            sum += num;
            println!("valid: {}", num);
        } else {
            println!("invalid: {}", number.as_int(&map));
        }
    }
    println!("SUM: {}", sum);
}

