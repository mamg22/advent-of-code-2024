use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use advent_of_code_2024::{grid_indices, grid_size, load_input, vector::Vector2d};

type AntennaIndex = HashMap<char, Vec<Vector2d<isize>>>;
type DeltaList = Vec<(Vector2d<isize>, Vector2d<isize>)>;

struct Map {
    width: usize,
    height: usize,
    index: AntennaIndex,
}

fn parse_map(input: &str) -> AntennaIndex {
    let mut antennas: AntennaIndex = HashMap::new();

    for (pos, ch) in grid_indices(input).filter(|idx| idx.1 != '.') {
        antennas.entry(ch).or_default().push(
            pos.try_components_into()
                .expect("Position convertible into signed"),
        )
    }

    antennas
}

fn in_bound(vector: Vector2d<isize>, width: usize, height: usize) -> bool {
    vector.x >= 0 && vector.x < width as isize && vector.y >= 0 && vector.y < height as isize
}

fn find_deltas(map: &Map) -> DeltaList {
    map.index
        .values()
        .flat_map(|antennas| antennas.iter().combinations(2))
        .map(|pair| (*pair[0], *pair[1] - *pair[0]))
        .collect()
}

fn find_antinodes(map: &Map, deltas: &DeltaList) -> HashSet<Vector2d<isize>> {
    let mut antinodes: HashSet<Vector2d<isize>> = HashSet::new();
    let width = map.width;
    let height = map.height;

    for (base, delta) in deltas {
        let anti0 = *base + delta.scalar_mul(-1);
        let anti1 = *base + delta.scalar_mul(2);

        if in_bound(anti0, width, height) {
            antinodes.insert(anti0);
        }
        if in_bound(anti1, width, height) {
            antinodes.insert(anti1);
        }
    }

    antinodes
}

fn find_resonant_antinodes(map: &Map, deltas: &DeltaList) -> HashSet<Vector2d<isize>> {
    let mut antinodes: HashSet<Vector2d<isize>> = HashSet::new();
    let width = map.width;
    let height = map.height;

    for (base, delta) in deltas {
        for factor in 0.. {
            let antinode = *base + delta.scalar_mul(factor);

            if in_bound(antinode, width, height) {
                antinodes.insert(antinode);
            } else {
                break;
            }
        }
        for factor in 1.. {
            let antinode = *base + delta.scalar_mul(-factor);

            if in_bound(antinode, width, height) {
                antinodes.insert(antinode);
            } else {
                break;
            }
        }
    }

    antinodes
}

fn main() {
    let input = load_input();
    let (width, height) = grid_size(&input);

    let index = parse_map(&input);

    let map = Map {
        width,
        height,
        index,
    };

    let deltas = find_deltas(&map);

    let antinodes = find_antinodes(&map, &deltas);
    let resonant_antinodes = find_resonant_antinodes(&map, &deltas);

    println!("Part 1: {}", antinodes.len());
    println!("Part 2: {}", resonant_antinodes.len());
}
