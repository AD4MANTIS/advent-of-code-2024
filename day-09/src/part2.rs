use std::cmp::Ordering;

use lib::ToVec;

lib::day!(09, part2, example => 2858, answer => 6_321_896_265_143);

struct Block {
    file_id: Option<usize>,
    size: usize,
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &[self.file_id]
                .repeat(self.size)
                .into_iter()
                .map(|x| x.map_or(".".to_string(), |x| x.to_string()))
                .to_vec()
                .join(""),
        )
    }
}

fn part2(input: &str) -> usize {
    let mut disc = decompress(&input[0..input.len() - 1]);
    fill_gaps_from_right_to_left(&mut disc);
    calculate_filesystem_checksum(disc)
}

fn decompress(disk_map: &str) -> Vec<Block> {
    let mut decompressed_file = Vec::with_capacity(disk_map.len());

    let mut current_file_id = 0;
    let mut is_file = true;

    for part in disk_map
        .chars()
        .map(|x| String::from(x).parse::<usize>().unwrap())
    {
        if is_file {
            decompressed_file.push(Block {
                file_id: Some(current_file_id),
                size: part,
            });

            current_file_id += 1;
        } else {
            decompressed_file.push(Block {
                file_id: None,
                size: part,
            });
        }

        is_file = !is_file;
    }

    decompressed_file
}

fn fill_gaps_from_right_to_left(decompressed_disc: &mut Vec<Block>) {
    let mut drained_from = decompressed_disc.len();

    while drained_from > 0 {
        drained_from -= 1;

        while drained_from > 0
            && decompressed_disc
                .get(drained_from)
                .map_or(false, |x| x.file_id.is_none())
        {
            drained_from -= 1;
        }

        let Some(block_to_move) = decompressed_disc.get(drained_from) else {
            continue;
        };

        let Some(filled_up_to) = decompressed_disc[0..drained_from]
            .iter()
            .position(|x| x.file_id.is_none() && x.size >= block_to_move.size)
        else {
            continue;
        };

        if filled_up_to >= drained_from {
            continue;
        }

        let Some(free_block) = decompressed_disc.get(filled_up_to) else {
            unreachable!();
        };

        match free_block.size.cmp(&block_to_move.size) {
            Ordering::Equal => decompressed_disc.swap(filled_up_to, drained_from),
            Ordering::Greater => {
                let space_left_in_free_block = free_block.size - block_to_move.size;
                decompressed_disc[filled_up_to].size = block_to_move.size;
                decompressed_disc.swap(filled_up_to, drained_from);
                decompressed_disc.insert(
                    filled_up_to + 1,
                    Block {
                        file_id: None,
                        size: space_left_in_free_block,
                    },
                );
                drained_from += 1;
            }
            Ordering::Less => {
                unreachable!();
            }
        }

        // dbg!(&decompressed_disc);
    }
}

fn calculate_filesystem_checksum(compacted_disc: Vec<Block>) -> usize {
    compacted_disc
        .into_iter()
        .flat_map(|block| [block.file_id].repeat(block.size))
        .enumerate()
        .filter_map(|(index, block)| block.map(|id| (index, id)))
        .map(|(index, file_block)| index * file_block.to_string().parse::<usize>().unwrap())
        .sum()
}
