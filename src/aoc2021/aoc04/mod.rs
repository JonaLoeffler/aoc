use std::num::ParseIntError;
use std::str::FromStr;

static INPUT: &'static str = include_str!("./input");

fn read() -> Game {
    Game::from_str(INPUT).unwrap()
}

#[derive(Debug)]
struct Board {
    grid: [[u32; 5]; 5],
    marked: [[bool; 5]; 5],
}

#[derive(Debug)]
struct Game {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Game {
    fn run(&mut self) -> Result<u32, &str> {
        for number in self.numbers.iter() {
            for board in self.boards.iter_mut() {
                board.mark_number(number);

                if board.is_complete() {
                    let result: u32 = *number * board.unmarked_numbers().iter().sum::<u32>();
                    return Ok(result);
                }
            }
        }

        Err("No board won!")
    }

    fn last(&mut self) -> Result<u32, &str> {
        for number in self.numbers.iter() {
            let mut last: Option<usize> = None;

            for (i, board) in self.boards.iter_mut().enumerate() {
                let incomplete = !board.is_complete();

                board.mark_number(number);

                if incomplete && board.is_complete() {
                    last = Some(i);
                }
            }

            if self.boards.iter().all(|b| b.is_complete()) && last != None {
                let score = self.boards[last.unwrap()]
                    .unmarked_numbers()
                    .iter()
                    .sum::<u32>();

                let result: u32 = *number * score;

                return Ok(result);
            }
        }

        Err("No board won!")
    }
}

impl Board {
    fn mark_number(&mut self, number: &u32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.grid[i][j] == *number {
                    self.marked[i][j] = true;
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        for i in 0..5 {
            let mut row = true;
            let mut col = true;

            for j in 0..5 {
                row = row && self.marked[i][j];
                col = col && self.marked[j][i];
            }

            if row || col {
                return true;
            }
        }

        false
    }

    fn unmarked_numbers(&self) -> Vec<u32> {
        let mut result = vec![];

        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    result.push(self.grid[i][j]);
                }
            }
        }

        result
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut grid = [[0; 5]; 5];

        for (i, line) in s.lines().enumerate() {
            for (j, n) in line
                .split(" ")
                .filter(|n| !n.trim().is_empty())
                .map(|n| n.trim().parse::<u32>().unwrap())
                .enumerate()
            {
                grid[i][j] = n;
            }
        }

        Ok(Board {
            grid,
            marked: [[false; 5]; 5],
        })
    }
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let numbers = s
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let boards: Vec<Board> = s
            .split("\n\n")
            .skip(1)
            .map(|b| Board::from_str(b).unwrap())
            .collect();

        Ok(Game { numbers, boards })
    }
}

pub fn one() -> i32 {
    let mut game = read();

    game.run().unwrap().try_into().unwrap()
}

pub fn two() -> i32 {
    let mut game = read();

    game.last().unwrap().try_into().unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_one() {
        assert_eq!(69579, super::one())
    }

    #[test]
    fn test_two() {
        assert_eq!(14877, super::two())
    }
}
