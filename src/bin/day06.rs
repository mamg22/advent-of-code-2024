use std::collections::HashSet;

use advent_of_code_2024::{
    direction::{AsVector, Direction},
    load_input,
    vector::Vector2d,
};

type Position = Vector2d<i16>;
type PositionSet = HashSet<Position>;

struct Map {
    width: i16,
    height: i16,
    obstacles: PositionSet,
    additional: Option<Position>,
}

impl Map {
    pub fn is_blocked(&self, position: Position) -> bool {
        self.obstacles.contains(&position) || self.additional.is_some_and(|pos| pos == position)
    }
}

#[derive(Debug, Clone)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    /// Return value tells if guard bonked
    fn do_move(&mut self, map: &Map) -> bool {
        let new_pos = self.next_position();

        if map.is_blocked(new_pos) {
            self.rotate();
            true
        } else {
            self.position = new_pos;
            false
        }
    }

    fn next_position(&self) -> Position {
        self.position + self.direction.as_vector()
    }

    fn rotate(&mut self) {
        self.direction = self.direction.rotate(1);
    }
}

fn follow_path(mut guard: Guard, map: &Map) -> Option<PositionSet> {
    let mut visited: PositionSet = HashSet::new();
    let mut bonked: HashSet<(Position, Direction)> = HashSet::new();

    loop {
        let pos = guard.position;

        if pos.x < 0 || pos.x >= map.width || pos.y < 0 || pos.y >= map.height {
            break Some(visited);
        }

        visited.insert(pos);

        // If the guard:
        //   moves and bonks against an obstacle
        //   and that obstacle has been bonked before
        //   then we're in an infinite loop
        if guard.do_move(&map) && !bonked.insert((guard.position, guard.direction)) {
            break None;
        }
    }
}

fn follow_path_no_track(mut guard: Guard, map: &Map) -> Option<()> {
    let mut bonked: HashSet<(Position, Direction)> = HashSet::new();

    loop {
        let pos = guard.position;

        if pos.x < 0 || pos.x >= map.width || pos.y < 0 || pos.y >= map.height {
            break None;
        }

        if guard.do_move(&map) && !bonked.insert((guard.position, guard.direction)) {
            break Some(());
        }
    }
}

fn possible_obstructions(guard: Guard, mut map: Map, visited: &PositionSet) -> usize {
    // We only try obstacles on visited tiles since those are the only that
    // will have an effect on the route.
    visited
        .iter()
        .filter(|pos| **pos != guard.position)
        .filter_map(|pos| {
            map.additional = Some(*pos);
            follow_path_no_track(guard.clone(), &map)
        })
        .count()
}

fn main() {
    let input = load_input();

    let width = input.lines().next().expect("Input non empty").len() as i16;
    let height = input.lines().count() as i16;

    let mut obstacles: PositionSet = HashSet::new();
    let mut guard: Option<Guard> = None;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.char_indices() {
            let position = Vector2d::new(x as i16, y as i16);

            if ch == '#' {
                obstacles.insert(position);
            } else if let Ok(direction) = Direction::from_char(ch) {
                guard = Some(Guard {
                    position,
                    direction,
                })
            }
        }
    }

    let map = Map {
        width,
        height,
        obstacles,
        additional: None,
    };

    let guard = guard.expect("Guard found");

    let visited = follow_path(guard.clone(), &map).expect("Regular path escapes");
    let possible = possible_obstructions(guard.clone(), map, &visited);

    println!("Part 1: {}", visited.len());
    println!("Part 2: {}", possible);
}
