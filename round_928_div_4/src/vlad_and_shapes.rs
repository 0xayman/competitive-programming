fn vlad_and_shapes(grid: Vec<Vec<u8>>, n: u32) -> bool {
    // for each row of the grid, count the number of 1s
    let mut row_count = vec![0; n as usize];
    for i in 0..n {
        for j in 0..n {
            if grid[i as usize][j as usize] == 1 {
                row_count[i as usize] += 1;
            }
        }
    }

    // remove the zeros
    row_count.retain(|&x| x != 0);

    // if all rows have the same number of 1s, then it's a square
    let is_square = row_count.iter().all(|&x| x == row_count[0]);

    is_square
}
