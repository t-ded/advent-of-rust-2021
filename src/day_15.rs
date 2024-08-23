use std::fs;
use std::io;
use pheap::PairingHeap;

const INF: u64 = u64::MAX;
const NEIGHBOURS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];


// From ChatGPT - TODO: Go through and understand!
fn read_input(filename: &str) -> io::Result<Vec<Vec<u32>>> {
    let content = fs::read_to_string(filename)?;
    let grid = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid input"))
                .collect()
        })
        .collect();
    Ok(grid)
}
// End of block from ChatGPT

fn maze_shortest_path(grid: &Vec<Vec<u32>>) -> u64 {
    let size = grid.len();

    let mut pqueue: PairingHeap<[i32; 2], u64> = PairingHeap::new();
    let mut memo: Vec<Vec<u64>> = vec![vec![INF; size]; size];
    memo[0][0] = 0;
    pqueue.insert([0, 0], 0);

    while !pqueue.is_empty() {
        if let Some(min_element) = pqueue.delete_min() {
            let min_node = min_element.0;
            for neighbouring_step in NEIGHBOURS {
                let new_x = min_node[0] + neighbouring_step[0];
                if new_x < 0 || new_x >= size as i32 { continue }
                let new_y = min_node[1] + neighbouring_step[1];
                if new_y < 0 || new_y >= size as i32 { continue }
                let possible_dist = memo[min_node[0] as usize][min_node[1] as usize] + grid[new_x as usize][new_y as usize] as u64;
                if possible_dist < memo[new_x as usize][new_y as usize] {
                    pqueue.insert([new_x, new_y], possible_dist);
                    memo[new_x as usize][new_y as usize] = possible_dist;
                }
            }
        };
    }

    memo[size - 1][size - 1]
}

fn expand_grid(original_grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let size = original_grid.len();
    let mut expanded_grid = vec![vec![0; size * 5]; size * 5];

    for i in 0..size * 5 {
        for j in 0..size * 5{
            expanded_grid[i][j] = (((original_grid[i % size][j % size] + ((i / size) as u32) + ((j / size) as u32)) - 1) % 9) + 1;
        }
    }

    expanded_grid
}

fn main() {
    if let Ok(grid) = read_input("input.txt") {
        println!("{}", maze_shortest_path(&grid));
        let expanded_grid = expand_grid(&grid);
        println!("{}", maze_shortest_path(&expanded_grid));
    };
}
