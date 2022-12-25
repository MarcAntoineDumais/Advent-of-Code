use std::fs;

pub fn part1() {
    run(1, 1);
}

pub fn part2() {
    run(811589153, 10);
}

fn run(decryption_key: i64, iteration: i64) {
    let data = fs::read_to_string("data/day20.txt").unwrap();
    let cypher: Vec<i64> = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>().unwrap() * decryption_key)
        .collect();
    let cypher_len = cypher.len() as i64;
    let mut data = cypher.clone();
    let mut indices: Vec<usize> = (0..(cypher_len as usize)).collect();

    // Mix
    for _ in 0..iteration {
        for (i, key) in cypher.iter().enumerate() {
            let data_index = indices[i] as i64;
            let key = ((*key % (cypher_len - 1)) + (cypher_len - 1)) % (cypher_len - 1);
            if key == 0 {
                continue;
            }
            let new_index = (data_index + key) % (cypher_len - 1);
            if data_index == new_index {
                continue;
            }

            let elem = data.remove(data_index as usize);
            data.insert(new_index as usize, elem);

            // update affected indices
            for j in 0usize..(cypher_len as usize) {
                if indices[j] == data_index as usize {
                    indices[j] = new_index as usize;
                    continue;
                }
                if data_index > new_index {
                    if indices[j] >= new_index as usize && indices[j] < data_index as usize {
                        indices[j] = ((indices[j] as i64 + 1) % cypher_len) as usize;
                    }
                } else {
                    if indices[j] > data_index as usize && indices[j] <= new_index as usize {
                        indices[j] = ((indices[j] as i64 - 1) % cypher_len) as usize;
                    }
                }
            }
        }
    }

    let zero_index = data.iter().position(|x| *x == 0).unwrap();
    let values = vec![
        data[(zero_index + 1000) % cypher_len as usize],
        data[(zero_index + 2000) % cypher_len as usize],
        data[(zero_index + 3000) % cypher_len as usize],
    ];
    println!(
        "{} + {} + {} = {}",
        values[0],
        values[1],
        values[2],
        values.iter().sum::<i64>()
    )
}
