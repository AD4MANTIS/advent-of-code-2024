lib::day!(09, part1, example => 1928, answer => 6_288_599_492_129);

fn part1(input: &str) -> usize {
    let mut disc = decompress(&input[0..input.len() - 1]);
    fill_gaps_from_right_to_left(&mut disc);
    calculate_filesystem_checksum(disc)
}

fn decompress(disk_map: &str) -> Vec<Option<usize>> {
    let mut decompressed_file = Vec::with_capacity(disk_map.len());

    let mut current_file_id = 0;
    let mut is_file = true;

    for part in disk_map
        .chars()
        .map(|x| String::from(x).parse::<usize>().unwrap())
    {
        if is_file {
            for _ in 0..part {
                decompressed_file.push(Some(current_file_id));
            }

            current_file_id += 1;
        } else {
            for _ in 0..part {
                decompressed_file.push(None);
            }
        }

        is_file = !is_file;
    }

    decompressed_file
}

fn fill_gaps_from_right_to_left(decompressed_disc: &mut [Option<usize>]) {
    let mut filled_up_to = 0;
    let mut drained_from = decompressed_disc.len() - 1;

    while filled_up_to <= drained_from {
        while decompressed_disc[filled_up_to].is_some() {
            filled_up_to += 1;
        }

        while decompressed_disc[drained_from].is_none() {
            drained_from -= 1;
        }

        decompressed_disc.swap(filled_up_to, drained_from);

        filled_up_to += 1;
        drained_from -= 1;
    }
}

fn calculate_filesystem_checksum(compacted_disc: Vec<Option<usize>>) -> usize {
    compacted_disc
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(index, file_block)| index * file_block.to_string().parse::<usize>().unwrap())
        .sum()
}
