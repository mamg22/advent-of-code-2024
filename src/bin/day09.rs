use std::collections::VecDeque;

use advent_of_code_2024::load_input;

#[derive(Debug)]
struct File {
    id: usize,
    blocks: VecDeque<usize>,
}

impl File {
    pub fn from_offset(id: usize, offset: usize, length: u8) -> Self {
        Self {
            id,
            blocks: VecDeque::from_iter((offset..).take(length as usize)),
        }
    }

    pub fn as_contiguous(&mut self) -> Result<(usize, usize), &str> {
        let contig = self.blocks.make_contiguous();

        if !contig.windows(2).all(|pair| pair[0].abs_diff(pair[1]) == 1) {
            Err("Non contiguous file")
        } else {
            let start = contig[0];
            let length = contig.len();
            Ok((start, length))
        }
    }

    pub fn move_to_contiguous(&mut self, start: usize) {
        let len = self.blocks.len();
        self.blocks.clear();
        self.blocks.extend((start..).take(len));
    }
}

fn parse_disk_map(disk: &str) -> (Vec<File>, VecDeque<(usize, usize)>) {
    let mut files: Vec<File> = Vec::new();
    let mut free: VecDeque<(usize, usize)> = VecDeque::new();

    let mut offset: usize = 0;

    for (idx, pair) in disk.as_bytes().chunks(2).enumerate() {
        let file_length: u8 = pair.get(0).map(|n| n - 0x30).expect("File in chunk");
        let file = File::from_offset(idx, offset, file_length);

        if file_length > 0 {
            files.push(file);
            offset += file_length as usize;
        }

        let Some(free_length) = pair.get(1).map(|n| n - 0x30) else {
            break;
        };

        if free_length > 0 {
            free.push_back((offset, free_length as usize));
            offset += free_length as usize;
        }
    }

    (files, free)
}

fn into_free_blocks(free_contiguous: VecDeque<(usize, usize)>) -> VecDeque<usize> {
    VecDeque::from_iter(
        free_contiguous
            .iter()
            .flat_map(|(start, length)| (*start..).take(*length)),
    )
}

fn checksum(files: &[File]) -> usize {
    files
        .iter()
        .map(|file| {
            file.blocks
                .iter()
                .map(|block| block * file.id)
                .sum::<usize>()
        })
        .sum()
}

fn defrag_check_blocks(disk_map: &str) -> usize {
    let (mut files, free) = parse_disk_map(&disk_map);
    let mut free = into_free_blocks(free);

    'outer: for file in files.iter_mut().rev() {
        for _ in 0..file.blocks.len() {
            let Some(new_block) = free.pop_front() else {
                break 'outer;
            };

            if new_block > file.blocks.back().cloned().unwrap_or(0) {
                break 'outer;
            }

            file.blocks.pop_back();
            file.blocks.push_front(new_block);
        }
    }

    checksum(&files)
}

fn defrag_check_contiguous(disk_map: &str) -> usize {
    let (mut files, mut free) = parse_disk_map(&disk_map);

    for file in files.iter_mut().rev() {
        let contig = file.as_contiguous().expect("Contiguous file");

        let Some(free_idx) = free.iter().position(|f| f.0 < contig.0 && f.1 >= contig.1) else {
            continue;
        };

        let target = free[free_idx];

        file.move_to_contiguous(target.0);

        let new_free = (target.0 + contig.1, target.1 - contig.1);

        if new_free.1 == 0 {
            free.remove(free_idx);
        } else {
            free[free_idx] = new_free;
        }
    }

    checksum(&files)
}

fn main() {
    let input = load_input();
    let input = input.trim_end();

    println!("Part 1: {}", defrag_check_blocks(&input));
    println!("Part 2: {}", defrag_check_contiguous(&input));
}
