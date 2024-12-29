use std::usize;

lib::day!(22, part1, example => 37327623, answer => 0);

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = calc_next_secret(secret);
            }

            secret
        })
        .sum()
}

fn calc_next_secret(mut secret: usize) -> usize {
    secret = mix(secret * 64, secret);
    secret = prune(secret);
    secret = mix(secret / 32, secret);
    secret = prune(secret);
    secret = mix(secret * 2048, secret);
    secret = prune(secret);

    secret
}

const fn mix(secret: usize, num: usize) -> usize {
    secret ^ num
}

const fn prune(secret: usize) -> usize {
    secret % 16777216
}
