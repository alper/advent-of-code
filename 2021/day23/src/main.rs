use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Burrow {
    hallway: [char; 11],
    // Each array is a room in order: A B C D
    // The 0-index element in a room is the bottom most spot
    rooms: [[char; 2]; 4],
}

impl Burrow {
    fn room_letter_to_index(&self, letter: char) -> usize {
        match letter {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            _ => 9,
        }
    }

    fn all_possible_moves(&self) -> Vec<Burrow> {
        let mut v = vec![];

        for i in 0..self.rooms.len() {
            let room = self.rooms[i];

            if room[0] == '.' && room[1] == '.' {
                continue;
            } else {
                let mut room_index = 1;

                if room[1] == '.' {
                    room_index = 0;
                }
                let hallway_spots = self.possible_hallway_spots(i);

                for spot in hallway_spots {
                    let new_burrow = self.swap_room_to_hall(i, room_index, spot);
                    v.push(new_burrow);
                }
            }
        }

        v
    }

    fn swap_room_to_hall(&self, room: usize, room_index: usize, hallway_index: usize) -> Burrow {
        let mut b2 = self.clone();

        let tmp = b2.hallway[hallway_index];
        b2.hallway[hallway_index] = b2.rooms[room][room_index];
        b2.rooms[room][room_index] = tmp;

        b2
    }

    fn possible_to_room(&self, hallway_index: usize, room_index: usize) -> usize {
        let letter = self.hallway[hallway_index];
        let correct_room = self.room_letter_to_index(letter);

        // Check if the letter is in range
        if !matches!(letter, 'A'..='D') {
            return 0;
        }

        // It needs to be the correct room
        if room_index != correct_room {
            return 0;
        }

        // Check room
        if self.rooms[room_index][0] != '.' && self.rooms[room_index][1] != '.' {
            // Full
            return 0;
        }

        // One Amphipod in the room already but the wrong one
        if self.rooms[room_index][0] != '.'
            && self.rooms[room_index][1] == '.'
            && self.rooms[room_index][0] != letter
        {
            return 0;
        }

        // Way there is not occluded?
        // Current position and destination room reduces the number of checks
        let room_entrance_index = match letter {
            'A' => 2,
            'B' => 4,
            'C' => 6,
            'D' => 8,
            _ => 100, // Out of bounds
        };

        let mut steps = 1;

        // Walk the steps between current position and room entrance
        if hallway_index > room_entrance_index {
            for r in room_entrance_index + 1..hallway_index {
                steps += 1;
                if self.hallway[r] != '.' {
                    return 0;
                }
            }
        } else {
            for r in hallway_index + 1..room_entrance_index {
                steps += 1;
                if self.hallway[r] != '.' {
                    return 0;
                }
            }
        }

        if self.rooms[room_index][0] == '.' && self.rooms[room_index][1] == '.' {
            steps += 2;
        } else {
            steps += 1;
        }

        steps
    }

    fn possible_hallway_spots(&self, room: usize) -> Vec<usize> {
        let mut v: Vec<usize> = vec![];

        match room {
            0 => {
                if self.hallway[1] == '.' {
                    v.push(1);
                }

                if self.hallway[1] == '.' && self.hallway[0] == '.' {
                    v.push(0);
                }

                if self.hallway[3] == '.' {
                    v.push(3);
                }

                if self.hallway[3] == '.' && self.hallway[5] == '.' {
                    v.push(5);
                }

                if self.hallway[3] == '.' && self.hallway[5] == '.' && self.hallway[7] == '.' {
                    v.push(7);
                }

                if self.hallway[3] == '.'
                    && self.hallway[5] == '.'
                    && self.hallway[7] == '.'
                    && self.hallway[9] == '.'
                {
                    v.push(9);
                }

                if self.hallway[3] == '.'
                    && self.hallway[5] == '.'
                    && self.hallway[7] == '.'
                    && self.hallway[9] == '.'
                    && self.hallway[10] == '.'
                {
                    v.push(10);
                }
            }
            1 => {
                // 3 / 1 / 0
                if self.hallway[3] == '.' {
                    v.push(3);
                }

                if self.hallway[3] == '.' && self.hallway[1] == '.' {
                    v.push(1);
                }

                if self.hallway[3] == '.' && self.hallway[1] == '.' && self.hallway[0] == '.' {
                    v.push(3);
                }

                // 5 / 7 / 9 / 10

                if self.hallway[5] == '.' {
                    v.push(5);
                }

                if self.hallway[5] == '.' && self.hallway[7] == '.' {
                    v.push(7);
                }

                if self.hallway[5] == '.' && self.hallway[7] == '.' && self.hallway[9] == '.' {
                    v.push(9);
                }

                if self.hallway[5] == '.'
                    && self.hallway[7] == '.'
                    && self.hallway[9] == '.'
                    && self.hallway[10] == '.'
                {
                    v.push(10);
                }
            }
            2 => {
                // 5 / 3 / 1 / 0
                if self.hallway[5] == '.' {
                    v.push(5);
                }

                if self.hallway[5] == '.' && self.hallway[3] == '.' {
                    v.push(3);
                }

                if self.hallway[5] == '.' && self.hallway[3] == '.' && self.hallway[1] == '.' {
                    v.push(1);
                }

                if self.hallway[5] == '.'
                    && self.hallway[3] == '.'
                    && self.hallway[1] == '.'
                    && self.hallway[0] == '.'
                {
                    v.push(0);
                }

                // 7 / 9 / 10
                if self.hallway[7] == '.' {
                    v.push(7);
                }

                if self.hallway[7] == '.' && self.hallway[9] == '.' {
                    v.push(9);
                }

                if self.hallway[7] == '.' && self.hallway[9] == '.' && self.hallway[10] == '.' {
                    v.push(10);
                }
            }
            3 => {
                // 7 / 5 / 3 / 1 0
                if self.hallway[7] == '.' {
                    v.push(7);
                }

                if self.hallway[7] == '.' && self.hallway[5] == '.' {
                    v.push(5);
                }

                if self.hallway[7] == '.' && self.hallway[5] == '.' && self.hallway[3] == '.' {
                    v.push(3);
                }

                if self.hallway[7] == '.'
                    && self.hallway[5] == '.'
                    && self.hallway[3] == '.'
                    && self.hallway[1] == '.'
                {
                    v.push(1);
                }

                if self.hallway[7] == '.'
                    && self.hallway[5] == '.'
                    && self.hallway[3] == '.'
                    && self.hallway[1] == '.'
                    && self.hallway[0] == '.'
                {
                    v.push(0);
                }

                // 9 / 10
                if self.hallway[9] == '.' {
                    v.push(9);
                }

                if self.hallway[9] == '.' && self.hallway[10] == '.' {
                    v.push(10);
                }
            }
            _ => {}
        }

        v.sort();

        v
    }
}

impl Debug for Burrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]\n  |{}|{}|{}|{}|\n  |{}|{}|{}|{}|",
            self.hallway.iter().collect::<String>(),
            self.rooms[0][1],
            self.rooms[1][1],
            self.rooms[2][1],
            self.rooms[3][1],
            self.rooms[0][0],
            self.rooms[1][0],
            self.rooms[2][0],
            self.rooms[3][0],
        )
    }
}

#[cfg(test)]
mod burrow_tests {
    use super::Burrow;

    #[test]
    fn test_possible_hallway_spots() {
        let b = Burrow {
            hallway: ['.'; 11],
            rooms: [['A', 'B'], ['D', 'C'], ['A', 'D'], ['C', 'B']],
        };

        assert_eq!(b.possible_hallway_spots(0), vec![0, 1, 3, 5, 7, 9, 10]);

        let b = Burrow {
            hallway: ['.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', '.'],
            rooms: [['A', 'B'], ['D', 'C'], ['A', 'D'], ['C', 'B']],
        };

        assert_eq!(b.possible_hallway_spots(1), vec![5, 7, 9, 10]);

        let b = Burrow {
            hallway: ['.', '.', '.', 'B', '.', '.', '.', 'B', '.', '.', '.'],
            rooms: [['A', 'B'], ['D', 'C'], ['A', 'D'], ['C', 'B']],
        };

        assert_eq!(b.possible_hallway_spots(2), vec![5]);
    }

    #[test]
    fn test_swap_room_to_hall() {
        let b = Burrow {
            hallway: ['.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', '.'],
            rooms: [['A', '.'], ['D', 'C'], ['A', 'D'], ['C', 'B']],
        };

        let c = b.swap_room_to_hall(1, 1, 10);

        assert_eq!(
            c,
            Burrow {
                hallway: ['.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', 'C'],
                rooms: [['A', '.'], ['D', '.'], ['A', 'D'], ['C', 'B']],
            }
        )
    }

    #[test]
    fn test_possible_to_room() {
        let b = Burrow {
            hallway: ['.', '.', '.', 'B', '.', '.', '.', 'D', '.', '.', '.'],
            rooms: [['A', '.'], ['D', 'C'], ['A', '.'], ['C', 'B']],
        };
        assert_eq!(b.possible_to_room(3, 3), 0);

        let b = Burrow {
            hallway: ['B', '.', '.', 'A', '.', '.', '.', 'D', '.', '.', '.'],
            rooms: [['.', '.'], ['D', 'C'], ['A', '.'], ['C', 'B']],
        };
        assert_eq!(b.possible_to_room(3, 0), 3);

        let b = Burrow {
            hallway: ['C', '.', '.', '.', '.', '.', '.', 'D', '.', '.', '.'],
            rooms: [['.', '.'], ['D', 'B'], ['C', '.'], ['A', 'B']],
        };
        assert_eq!(b.possible_to_room(0, 2), 7);
    }
}

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    let b = Burrow {
        hallway: ['.'; 11],
        rooms: [['A', 'B'], ['D', 'C'], ['A', 'D'], ['C', 'B']],
    };

    println!("{:?}", b);

    let moves = b.all_possible_moves();

    println!("Moves");
    for m in moves {
        println!("{:?}", m);
        println!();
    }
}
