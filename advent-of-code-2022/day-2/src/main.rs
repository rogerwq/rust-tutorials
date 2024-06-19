//! Rules
//! 
//! A, X: Rock
//! B, Y: Paper
//! C, Z: Scissor
//! 
//! rock +1, paper +2, scissor +3
//! losing 0, drawing +3, winning +6
//! 
//! A Y +2+6
//! B X +1+0
//! C Z +3+3
//! total: 15

use std::str::FromStr;

// use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissor
}

impl Move {
    fn select_points(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }

    fn win(&self, theirs: &Move) -> bool {
        matches!((self, theirs), (Move::Rock, Move::Scissor) | (Move::Paper, Move::Rock) | (Move::Scissor, Move::Paper))
    }

    fn outcome(&self, theirs: &Move) -> Outcome {
        if self.win(theirs) {
            Outcome::Win
        } else if theirs.win(self) {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }

    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissor];

    fn lose_move(&self) -> Self {
        Self::ALL_MOVES.iter()
            .copied()
            .find(|m| m.win(self))
            .unwrap()
    }

    fn win_move(&self) -> Self {
        Self::ALL_MOVES.iter()
            .copied()
            .find(|m| self.win(m))
            .unwrap()
    }
}

impl TryFrom<char> for Move {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            // 'A' | 'X' => Ok(Move::Rock),
            // 'B' | 'Y' => Ok(Move::Paper),
            // 'C' | 'Z' => Ok(Move::Scissor),
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissor),
            _ => Err(format!("{value} is invalid char for Move"))
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lose
}

impl TryFrom<char> for Outcome {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(format!("{value} is invalid char for Outcome"))
        }
    }
}

impl Outcome {
    fn points(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0
        }
    }
}


#[derive(Debug)]
struct Round {
    theirs: Move,
    ours: Move
}

impl FromStr for Round {
    type Err = String;

    // fn from_str(s: &str) -> Result<Self, Self::Err> {
    //     let mut chars = s.chars();
    //     let (Some(theirs), Some(' '), Some(ours), None) = (
    //         chars.next(),
    //         chars.next(),
    //         chars.next(),
    //         chars.next(),
    //     ) else {
    //         return Err(String::from("input line error"));
    //     };

    //     let theirs = Move::try_from(theirs)?;
    //     let ours = Move::try_from(ours)?;

    //     Ok(Self { theirs, ours })
    // }

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(outcome), None) = (
            chars.next(),
            chars.next(),
            chars.next(),
            chars.next(),
        ) else {
            return Err(String::from("input line error"));
        };

        let theirs = Move::try_from(theirs)?;
        let outcome = Outcome::try_from(outcome)?;
        let ours = match outcome {
            Outcome::Draw => theirs,
            Outcome::Win => theirs.lose_move(),
            Outcome::Lose => theirs.win_move()
        };

        Ok(Self { theirs, ours })
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        self.ours.outcome(&self.theirs)
    }

    fn points(&self) -> usize {
        self.ours.select_points() + self.outcome().points()
    }
}

fn main() {
    // let points: usize = include_str!("input.txt").lines()
    //     // .map(|line| Round::from_str(line).unwrap())
    //     .map(Round::from_str)
    //     // .map(|line| line.parse::<Round>().unwrap())
    //     .map(|round| round.unwrap().points())
    //     .sum();

    // use filter_map
    //
    let points: usize = include_str!("input.txt").lines()
        .filter_map(|line| Round::from_str(line).ok())
        .map(|round| round.points())
        .sum();

    dbg!(points);
}

// fn main() {
//     // use itertools::process_results
//     //
//     // let nums: Vec<Result<usize, &str>> = vec![Ok(1), Ok(2), Ok(3), Err("error")];
//     // let points = itertools::process_results(
//     //     nums.into_iter(), 
//     //     |it| it.sum::<usize>()
//     // );
//     let points: usize = itertools::process_results(
//         include_str!("input.txt").lines().map(Round::from_str)
//             .map(|result_round| result_round.map(|round| round.points())),
//          |it| it.sum::<usize>()
//     ).unwrap();

//     // use map_ok
//     //
//     let points: usize = itertools::process_results(
//         include_str!("input.txt").lines().map(Round::from_str)
//             .map_ok(|round| round.points()),
//          |it| it.sum::<usize>()
//     ).unwrap();

//     dbg!(points);
// }


// Rule
//     A, B, C => Rock, Paper, Scissor
//     X, Y, Z => Loss, Draw, Win
// 
// A Y => ours: A +1+3 
// B X => ours: A +1+0
// C Z => ours: A +1+6
// total: 12
//