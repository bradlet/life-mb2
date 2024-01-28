//! life.rs
//! Game of Life Library provided by Bart Massey

/// Return `true` iff all cells are off.
#[allow(dead_code)]
pub fn done(fb: &[[u8; 5]; 5]) -> bool {
    fb.iter().all(|&row| row.iter().all(|&v| v == 0))
}

/// Run one turn of Life.
#[allow(dead_code)]
pub fn life(fb: &mut [[u8; 5]; 5]) {
    let prev = *fb;
    for row in 0..5 {
        for col in 0..5 {
            let prev_row = (row + 4) % 5;
            let next_row = (row + 1) % 5;
            let prev_col = (col + 4) % 5;
            let next_col = (col + 1) % 5;
            let coords = [
                (prev_row, prev_col),
                (prev_row, col),
                (prev_row, next_col),
                (row, prev_col),
                (row, next_col),
                (next_row, prev_col),
                (next_row, col),
                (next_row, next_col),
            ];
            let neighbors = coords.into_iter().map(|(r, c)| prev[r][c]).sum();
            #[allow(clippy::manual_range_contains)]
            match (prev[row][col], neighbors) {
                (1, n) if n < 2 || n > 3 => fb[row][col] = 0,
                (0, 3) => fb[row][col] = 1,
                (_, _) => (),
            }
        }
    }
}
