use pathfinding::prelude::dijkstra;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Burrow {
    hallway: [char; 11],
    // Each array is a room in order: A B C D
    // The 0-index element in a room is the bottom most spot
    rooms: [[char; 4]; 4],
}

impl Burrow {
    fn room_letter_to_number(letter: char) -> usize {
        match letter {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            _ => 9,
        }
    }

    fn amphipod_cost_factor(letter: char) -> usize {
        match letter {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => 0,
        }
    }

    fn room_number_to_letter(index: usize) -> char {
        match index {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            _ => '.',
        }
    }

    fn number_of_free_spots(&self, room_number: usize) -> usize {
        self.rooms[room_number]
            .iter()
            .filter(|c| **c == '.')
            .count()
    }

    fn last_amphipod_index(&self, room_number: usize) -> Option<usize> {
        match self.number_of_free_spots(room_number) {
            0 => Some(3),
            1 => Some(2),
            2 => Some(1),
            3 => Some(0),
            _ => None, // Colud mean it's empty
        }
    }

    fn first_free_index(&self, room_number: usize) -> Option<usize> {
        match self.number_of_free_spots(room_number) {
            0 => None,
            1 => Some(3),
            2 => Some(2),
            3 => Some(1),
            4 => Some(0),
            _ => None,
        }
    }

    fn all_possible_moves(&self) -> Vec<(Burrow, usize)> {
        let mut v = vec![];

        for room_number in 0..self.rooms.len() {
            let room = self.rooms[room_number];

            if self.number_of_free_spots(room_number) == 4 {
                continue; // No amphipods in this room to move out
            } else {
                let hallway_spots = self.possible_hallway_spots(room_number);

                let amphipod_to_move_index = self.last_amphipod_index(room_number).unwrap(); // We know it's not empty because of the other if branch

                let amphipod = room[amphipod_to_move_index];
                let cost_factor = Burrow::amphipod_cost_factor(amphipod);
                let mut steps_addition = 0;

                if amphipod_to_move_index == 0 {
                    steps_addition = 3;
                } else if amphipod_to_move_index == 1 {
                    steps_addition = 2;
                } else if amphipod_to_move_index == 2 {
                    steps_addition = 1;
                }

                for (spot, steps) in hallway_spots {
                    let new_burrow =
                        self.swap_room_to_hall(room_number, amphipod_to_move_index, spot);

                    v.push((new_burrow, (steps + steps_addition) * cost_factor));
                }
            }
        }

        // TODO also cover the case where an amphipod can move from the wrong room to the right room at once?
        // For now ignore that.

        for hallway_index in 0..self.hallway.len() {
            let amphipod = self.hallway[hallway_index];

            if amphipod != '.' {
                let destination_room_number = Burrow::room_letter_to_number(amphipod);

                let steps = self.possible_to_room(hallway_index, destination_room_number);

                if steps > 0 {
                    let room_index = self.first_free_index(destination_room_number).unwrap();

                    let new_burrow =
                        self.swap_room_to_hall(destination_room_number, room_index, hallway_index);

                    let move_cost = Burrow::amphipod_cost_factor(amphipod) * steps;
                    v.push((new_burrow, move_cost));
                }
            }
        }

        v
    }

    fn swap_room_to_hall(&self, room: usize, room_index: usize, hallway_index: usize) -> Burrow {
        let mut b2 = *self;

        std::mem::swap(
            &mut b2.hallway[hallway_index],
            &mut b2.rooms[room][room_index],
        );

        b2
    }

    /// Checks if it's possible to move to a room from the hallway and returns how many steps
    /// that would take
    fn possible_to_room(&self, hallway_index: usize, room_number: usize) -> usize {
        let letter = self.hallway[hallway_index];
        let correct_room = Burrow::room_letter_to_number(letter);

        // Check if the letter is in range
        if !matches!(letter, 'A'..='D') {
            return 0;
        }

        // It needs to be the correct room
        if room_number != correct_room {
            return 0;
        }

        // Check room for only correct amphipods (which means there will be freespots)
        for i in 0..self.rooms[room_number].len() {
            if self.rooms[room_number][i] != letter && self.rooms[room_number][i] != '.' {
                return 0;
            }
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

        // Walk the steps between current position and room entrance and check for occlusion
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

        // TODO replace with self.last_amphipod
        if self.rooms[room_number][0] == '.'
            && self.rooms[room_number][1] == '.'
            && self.rooms[room_number][2] == '.'
            && self.rooms[room_number][3] == '.'
        {
            steps += 4;
        } else if self.rooms[room_number][1] == '.'
            && self.rooms[room_number][2] == '.'
            && self.rooms[room_number][3] == '.'
        {
            steps += 3;
        } else if self.rooms[room_number][2] == '.' && self.rooms[room_number][3] == '.' {
            steps += 2;
        } else {
            steps += 1;
        }

        steps
    }

    /// Returns a vec with a 2-tuple with the index of the hallway position
    /// you can reach and the steps necessary to get there from the room_index=3
    /// (the spot in the room closest to the hallway)
    fn possible_hallway_spots(&self, room_number: usize) -> Vec<(usize, usize)> {
        let mut v: Vec<(usize, usize)> = vec![];

        // Check whether the amphipods in this room are already at their destination
        let correct_amphipod_letter = Burrow::room_number_to_letter(room_number);

        // Either two of the correct
        if self.rooms[room_number][0] == correct_amphipod_letter
            && self.rooms[room_number][1] == correct_amphipod_letter
        {
            return v;
        }

        // Or one of the correct at the bottom in which case they don't move either
        if self.rooms[room_number][0] == correct_amphipod_letter
            && self.rooms[room_number][1] == '.'
        {
            return v;
        }

        match room_number {
            0 => {
                if self.hallway[1] == '.' {
                    v.push((1, 2));
                }

                if self.hallway[1] == '.' && self.hallway[0] == '.' {
                    v.push((0, 3));
                }

                if self.hallway[3] == '.' {
                    v.push((3, 2));
                }

                if self.hallway[3] == '.' && self.hallway[5] == '.' {
                    v.push((5, 4));
                }

                if self.hallway[3] == '.' && self.hallway[5] == '.' && self.hallway[7] == '.' {
                    v.push((7, 6));
                }

                if self.hallway[3] == '.'
                    && self.hallway[5] == '.'
                    && self.hallway[7] == '.'
                    && self.hallway[9] == '.'
                {
                    v.push((9, 8));
                }

                if self.hallway[3] == '.'
                    && self.hallway[5] == '.'
                    && self.hallway[7] == '.'
                    && self.hallway[9] == '.'
                    && self.hallway[10] == '.'
                {
                    v.push((10, 9));
                }
            }
            1 => {
                // 3 / 1 / 0
                if self.hallway[3] == '.' {
                    v.push((3, 2));
                }

                if self.hallway[3] == '.' && self.hallway[1] == '.' {
                    v.push((1, 4));
                }

                if self.hallway[3] == '.' && self.hallway[1] == '.' && self.hallway[0] == '.' {
                    v.push((3, 5));
                }

                // 5 / 7 / 9 / 10

                if self.hallway[5] == '.' {
                    v.push((5, 2));
                }

                if self.hallway[5] == '.' && self.hallway[7] == '.' {
                    v.push((7, 4));
                }

                if self.hallway[5] == '.' && self.hallway[7] == '.' && self.hallway[9] == '.' {
                    v.push((9, 6));
                }

                if self.hallway[5] == '.'
                    && self.hallway[7] == '.'
                    && self.hallway[9] == '.'
                    && self.hallway[10] == '.'
                {
                    v.push((10, 7));
                }
            }
            2 => {
                // 5 / 3 / 1 / 0
                if self.hallway[5] == '.' {
                    v.push((5, 2));
                }

                if self.hallway[5] == '.' && self.hallway[3] == '.' {
                    v.push((3, 4));
                }

                if self.hallway[5] == '.' && self.hallway[3] == '.' && self.hallway[1] == '.' {
                    v.push((1, 6));
                }

                if self.hallway[5] == '.'
                    && self.hallway[3] == '.'
                    && self.hallway[1] == '.'
                    && self.hallway[0] == '.'
                {
                    v.push((0, 7));
                }

                // 7 / 9 / 10
                if self.hallway[7] == '.' {
                    v.push((7, 2));
                }

                if self.hallway[7] == '.' && self.hallway[9] == '.' {
                    v.push((9, 4));
                }

                if self.hallway[7] == '.' && self.hallway[9] == '.' && self.hallway[10] == '.' {
                    v.push((10, 5));
                }
            }
            3 => {
                // 7 / 5 / 3 / 1 0
                if self.hallway[7] == '.' {
                    v.push((7, 2));
                }

                if self.hallway[7] == '.' && self.hallway[5] == '.' {
                    v.push((5, 4));
                }

                if self.hallway[7] == '.' && self.hallway[5] == '.' && self.hallway[3] == '.' {
                    v.push((3, 6));
                }

                if self.hallway[7] == '.'
                    && self.hallway[5] == '.'
                    && self.hallway[3] == '.'
                    && self.hallway[1] == '.'
                {
                    v.push((1, 8));
                }

                if self.hallway[7] == '.'
                    && self.hallway[5] == '.'
                    && self.hallway[3] == '.'
                    && self.hallway[1] == '.'
                    && self.hallway[0] == '.'
                {
                    v.push((0, 9));
                }

                // 9 / 10
                if self.hallway[9] == '.' {
                    v.push((9, 2));
                }

                if self.hallway[9] == '.' && self.hallway[10] == '.' {
                    v.push((10, 3));
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
            "[{}]\n  |{}|{}|{}|{}|\n  |{}|{}|{}|{}|\n  |{}|{}|{}|{}|\n  |{}|{}|{}|{}|",
            self.hallway.iter().collect::<String>(),
            self.rooms[0][3],
            self.rooms[1][3],
            self.rooms[2][3],
            self.rooms[3][3],
            self.rooms[0][2],
            self.rooms[1][2],
            self.rooms[2][2],
            self.rooms[3][2],
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
            rooms: [
                ['A', 'D', 'D', 'B'],
                ['D', 'B', 'C', 'C'],
                ['C', 'A', 'B', 'B'],
                ['A', 'C', 'A', 'D'],
            ],
        };

        println!("Spots");

        assert_eq!(
            b.possible_hallway_spots(0),
            vec![(0, 3), (1, 2), (3, 2), (5, 4), (7, 6), (9, 8), (10, 9)]
        );

        // let b = Burrow {
        //     hallway: ['.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', '.'],
        //     rooms: [['A', 'B'], ['D', 'C'], ['A', 'D'], ['C', 'B']],
        // };

        // assert_eq!(
        //     b.possible_hallway_spots(1),
        //     vec![(5, 1), (7, 1), (9, 1), (10, 1)]
        // );

        // let b = Burrow {
        //     hallway: ['.', '.', '.', 'B', '.', '.', '.', 'B', '.', '.', '.'],
        //     rooms: [['A', 'B'], ['D', 'C'], ['A', 'D'], ['C', 'B']],
        // };

        // assert_eq!(b.possible_hallway_spots(2), vec![(5, 1)]);
    }

    #[test]
    fn test_number_of_free_spots() {
        let b = Burrow {
            hallway: ['.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', '.'],
            rooms: [
                ['A', 'D', 'D', '.'],
                ['D', 'B', '.', '.'],
                ['C', '.', '.', '.'],
                ['A', 'C', 'A', 'D'],
            ],
        };

        assert_eq!(b.number_of_free_spots(0), 1);
        assert_eq!(b.number_of_free_spots(1), 2);
        assert_eq!(b.number_of_free_spots(2), 3);
        assert_eq!(b.number_of_free_spots(3), 0);
    }

    #[test]
    fn test_swap_room_to_hall() {
        let b = Burrow {
            hallway: ['.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', '.'],
            rooms: [
                ['A', 'D', 'D', '.'],
                ['D', 'B', 'C', 'C'],
                ['C', 'A', 'B', 'B'],
                ['A', 'C', 'A', 'D'],
            ],
        };

        let c = b.swap_room_to_hall(1, 3, 10);

        assert_eq!(
            c,
            Burrow {
                hallway: ['.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', 'C'],
                rooms: [
                    ['A', 'D', 'D', '.'],
                    ['D', 'B', 'C', '.'],
                    ['C', 'A', 'B', 'B'],
                    ['A', 'C', 'A', 'D'],
                ],
            }
        )
    }

    #[test]
    fn test_possible_to_room() {
        let b = Burrow {
            hallway: ['.', '.', '.', 'B', '.', '.', '.', 'D', '.', '.', '.'],
            rooms: [
                ['A', 'A', 'A', '.'],
                ['D', 'C', 'D', 'C'],
                ['A', '.', '.', '.'],
                ['C', 'B', 'C', 'B'],
            ],
        };
        // println!("{:?}", b);

        assert_eq!(b.possible_to_room(3, 3), 0);

        let b = Burrow {
            hallway: ['.', '.', '.', 'A', '.', '.', '.', 'D', '.', '.', '.'],
            rooms: [
                ['A', 'A', 'A', '.'],
                ['D', 'C', 'D', 'C'],
                ['A', '.', '.', '.'],
                ['C', 'B', 'C', 'B'],
            ],
        };
        // println!("{:?}", b);

        assert_eq!(b.possible_to_room(3, 0), 2);

        let b = Burrow {
            hallway: ['.', '.', '.', 'C', '.', '.', '.', 'D', '.', '.', '.'],
            rooms: [
                ['A', 'A', 'A', '.'],
                ['D', 'C', 'D', 'C'],
                ['C', '.', '.', '.'],
                ['C', 'B', 'C', 'B'],
            ],
        };
        println!("{:?}", b);

        assert_eq!(b.possible_to_room(3, 2), 6);
    }

    #[test]
    fn test_all_possible_moves() {
        let b = Burrow {
            hallway: ['.', '.', '.', 'A', '.', '.', '.', 'D', '.', '.', '.'],
            rooms: [
                ['A', 'A', 'A', '.'],
                ['D', 'C', 'D', 'C'],
                ['.', '.', '.', '.'],
                ['C', 'B', 'C', 'B'],
            ],
        };
        println!("Input:");
        println!("{:?}", b);
        println!();

        for (b, cost) in b.all_possible_moves() {
            println!("{:?}", b);
            println!("Cost: {}", cost);
            println!();
        }
    }
}

fn main() {
    let test_input = Burrow {
        hallway: ['.'; 11],
        rooms: [
            ['A', 'D', 'D', 'B'],
            ['D', 'B', 'C', 'C'],
            ['C', 'A', 'B', 'B'],
            ['A', 'C', 'A', 'D'],
        ],
    };

    let puzzle_input = Burrow {
        hallway: ['.'; 11],
        rooms: [
            ['D', 'D', 'D', 'C'],
            ['D', 'B', 'C', 'C'],
            ['B', 'A', 'B', 'A'],
            ['A', 'C', 'A', 'B'],
        ],
    };

    let goal = Burrow {
        hallway: ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        rooms: [
            ['A', 'A', 'A', 'A'],
            ['B', 'B', 'B', 'B'],
            ['C', 'C', 'C', 'C'],
            ['D', 'D', 'D', 'D'],
        ],
    };

    let result = dijkstra(&puzzle_input, |&b| b.all_possible_moves(), |c| *c == goal);
    println!("Result: {:?}", result);
    let (path, path_cost) = result.unwrap();

    for m in path {
        println!("{:?}", m);
    }

    println!("Cost: {}", path_cost);
}
