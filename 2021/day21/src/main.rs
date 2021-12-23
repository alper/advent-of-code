use itertools::Itertools;
use std::{collections::HashMap, fs};

#[derive(Debug, Copy, Clone)]
enum OneOrTwo {
    One,
    Two,
}

#[derive(Debug, Copy, Clone)]
struct Player {
    pos: usize,
    score: usize,
    one_or_two: OneOrTwo,
}

struct DeterministicDie {
    curr: usize,
    rolls: usize,
}

impl Iterator for DeterministicDie {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.rolls += 1;

        let old_curr = self.curr;

        self.curr += 1;

        if self.curr > 100 {
            self.curr -= 100;
        }

        Some(old_curr)
    }
}

fn main() {
    part2_alt();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct UniverseState {
    p1pos: u128,
    p1score: u128,
    p2pos: u128,
    p2score: u128,
    turn: u128, // Whic player's turn it is
}

const DIE_ROLLS: [(u128, u128); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn part2_alt() {
    let mut win1: u128 = 0;
    let mut win2: u128 = 0;

    dice(8, 0, &mut win1, 6, 0, &mut win2, 1);

    println!("Win 1: {}, Win 2: {}", win1, win2);
}

fn dice(
    pos1: u128,
    score1: u128,
    win1: &mut u128,
    pos2: u128,
    score2: u128,
    win2: &mut u128,
    score_mult: u128,
) {
    // println!("Score {} {}", win1, win2);

    if score2 >= 21 {
        *win2 += score_mult;
        // println!("Score added: {}", score_mult);
    } else {
        for roll in DIE_ROLLS {
            let new_pos = ((pos1 + roll.0 - 1) % 10) + 1;

            dice(
                pos2,
                score2,
                win2,
                new_pos,
                score1 + new_pos,
                win1,
                score_mult * roll.1,
            );
        }
    }
}

fn part2() {
    let test_start = (4, 8);
    let input = test_start;

    let mut universe_counts: HashMap<UniverseState, u128> = HashMap::new();
    universe_counts.insert(
        UniverseState {
            p1pos: input.0,
            p1score: 0,
            p2pos: input.1,
            p2score: 0,
            turn: 1,
        },
        1,
    );

    let mut player1Wins = 0 as u128;
    let mut player2Wins = 0 as u128;

    loop {
        let mut new_universe_counts: HashMap<UniverseState, u128> = HashMap::new();

        for (&s, &count) in universe_counts.iter() {
            // println!("State: {:?} count {}", s, count);

            for (roll, freq) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                if s.turn == 1 {
                    let s_new = UniverseState {
                        p1pos: (s.p1pos + roll) % 10,
                        p1score: s.p1score + ((s.p1pos + roll) % 10) + 1,
                        p2pos: s.p2pos,
                        p2score: s.p2score,
                        turn: 2,
                    };

                    if s_new.p1score >= 21 {
                        player1Wins += count * freq;
                    } else {
                        *new_universe_counts.entry(s_new).or_insert(count) *= freq;
                    }
                } else {
                    let s_new = UniverseState {
                        p2pos: (s.p2pos + roll) % 10,
                        p2score: s.p2score + ((s.p2pos + roll) % 10) + 1,
                        p1pos: s.p1pos,
                        p1score: s.p1score,
                        turn: 1,
                    };

                    if s_new.p2score >= 21 {
                        player2Wins += count * freq;
                    } else {
                        *new_universe_counts.entry(s_new).or_insert(count) *= freq;
                    }
                }
            }
        }

        println!("Univers: {:?}", universe_counts);
        // println!("New Univers: {:?}", new_universe_counts);
        universe_counts = new_universe_counts;

        if universe_counts.len() == 0 {
            break;
        }
    }

    println!(
        "Playr 1 wins {}, Player 2 wins {}",
        player1Wins, player2Wins
    );
}

fn part1() {
    let test_start = (4, 8);
    let real_start = (8, 6);

    let input = real_start;

    let mut d = DeterministicDie { curr: 1, rolls: 0 };

    let mut player1 = Player {
        pos: input.0,
        score: 0,
        one_or_two: OneOrTwo::One,
    };
    let mut player2 = Player {
        pos: input.1,
        score: 0,
        one_or_two: OneOrTwo::Two,
    };

    let mut current_player = &mut player1;

    loop {
        let roll1 = d.next().unwrap();
        let roll2 = d.next().unwrap();
        let roll3 = d.next().unwrap();

        let new_pos = calculate_pos(current_player.pos, roll1 + roll2 + roll3);

        current_player.score += new_pos;
        current_player.pos = new_pos;

        // println!(
        //     "Player: {:?} Score: {}",
        //     current_player.one_or_two, current_player.score
        // );

        if current_player.score >= 1000 {
            break;
        }

        // Switch players
        match current_player.one_or_two {
            OneOrTwo::One => {
                current_player = &mut player2;
            }
            OneOrTwo::Two => {
                current_player = &mut player1;
            }
        }
    }

    println!(
        "Answer 1: {}",
        std::cmp::min(player1.score, player2.score) * d.rolls
    );
}

#[test]
fn test_calculate_space() {
    assert_eq!(calculate_pos(9, 2), 1);
    assert_eq!(calculate_pos(7, 8), 5);
    assert_eq!(calculate_pos(9, 23), 2);
}

fn calculate_pos(pos: usize, roll: usize) -> usize {
    let mut new_pos = pos + roll;

    while new_pos > 10 {
        new_pos -= 10;
    }

    new_pos
}
