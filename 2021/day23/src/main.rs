use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

    fn amphipod_cost_factor(letter: char) -> usize {
        match letter {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => 0,
        }
    }

    fn room_index_to_letter(&self, index: usize) -> char {
        match index {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            _ => '.',
        }
    }

    fn all_possible_moves(&self) -> Vec<(Burrow, usize)> {
        let mut v = vec![];

        for i in 0..self.rooms.len() {
            let room = self.rooms[i];

            if room[0] == '.' && room[1] == '.' {
                continue;
            } else {
                let hallway_spots = self.possible_hallway_spots(i);

                // TODO move this free spot in room checking into the `swap_room_to_hall` function?
                let mut room_index = 1;

                if room[1] == '.' {
                    room_index = 0;
                }

                let amphipod = room[room_index];
                let cost_factor = Burrow::amphipod_cost_factor(amphipod);
                let mut steps_addition = 0;
                if room_index == 0 {
                    steps_addition = 1;
                }

                for (spot, steps) in hallway_spots {
                    let new_burrow = self.swap_room_to_hall(i, room_index, spot);

                    v.push((new_burrow, (steps + steps_addition) * cost_factor));
                }
            }
        }

        // TODO also cover the case where an amphipod can move from the wrong room to the right room at once?
        // For now ignore that.

        for hallway_index in 0..self.hallway.len() {
            let amphipod = self.hallway[hallway_index];

            if amphipod != '.' {
                let destination_room_index = self.room_letter_to_index(amphipod);

                let steps = self.possible_to_room(hallway_index, destination_room_index);

                if steps > 0 {
                    let mut room_index = 0;
                    if self.rooms[destination_room_index][0] != '.' {
                        room_index = 1;
                    }

                    let new_burrow =
                        self.swap_room_to_hall(destination_room_index, room_index, hallway_index);

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

    /// Returns a vec with a 2-tuple with the index of the hallway position
    /// you can reach and the steps necessary to get there from the room_index=1
    fn possible_hallway_spots(&self, room_number: usize) -> Vec<(usize, usize)> {
        let mut v: Vec<(usize, usize)> = vec![];

        // Check whether the amphipods in this room are already at their destination
        let correct_amphipod_letter = self.room_index_to_letter(room_number);

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

// impl Hash for Burrow {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {}
// }

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

        assert_eq!(
            b.possible_hallway_spots(0),
            vec![(0, 0), (1, 1), (3, 3), (5, 1), (7, 1), (9, 1), (10, 1)]
        );

        let b = Burrow {
            hallway: ['.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', '.'],
            rooms: [['A', 'B'], ['D', 'C'], ['A', 'D'], ['C', 'B']],
        };

        assert_eq!(
            b.possible_hallway_spots(1),
            vec![(5, 1), (7, 1), (9, 1), (10, 1)]
        );

        let b = Burrow {
            hallway: ['.', '.', '.', 'B', '.', '.', '.', 'B', '.', '.', '.'],
            rooms: [['A', 'B'], ['D', 'C'], ['A', 'D'], ['C', 'B']],
        };

        assert_eq!(b.possible_hallway_spots(2), vec![(5, 1)]);
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
    let test_input = Burrow {
        hallway: ['.'; 11],
        rooms: [['A', 'B'], ['D', 'C'], ['C', 'B'], ['A', 'D']],
    };

    let puzzle_input = Burrow {
        hallway: ['.'; 11],
        rooms: [['D', 'C'], ['D', 'C'], ['B', 'A'], ['A', 'B']],
    };
    // let b = Burrow {
    //     hallway: ['.', '.', '.', '.', '.', 'D', '.', '.', '.', '.', '.'],
    //     rooms: [['A', '.'], ['B', 'B'], ['C', 'C'], ['A', 'D']],
    // };
    // let b = Burrow {
    //     hallway: ['.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', '.'],
    //     rooms: [['A', 'B'], ['D', 'C'], ['C', '.'], ['A', 'D']],
    // };

    // println!("{:?}", b);

    // let moves = b.all_possible_moves();

    // println!("Moves");
    // for m in moves {
    //     println!("{:?}", m);
    //     println!();
    // }

    let goal = Burrow {
        hallway: ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        rooms: [['A', 'A'], ['B', 'B'], ['C', 'C'], ['D', 'D']],
    };

    let result = dijkstra(&puzzle_input, |&b| b.all_possible_moves(), |c| *c == goal);

    let (path, path_cost) = result.unwrap();

    for m in path {
        println!("{:?}", m);
    }

    println!("Cost: {}", path_cost);
}
