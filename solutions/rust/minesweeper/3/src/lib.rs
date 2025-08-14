pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut counts: Vec<Vec<char>> = minefield.iter().map(|&row| row.chars().collect()).collect();

    for y in 0..counts.len() {
        for x in 0..counts[y].len() {
            let cur_cell = counts[y][x];
            if cur_cell == ' ' {
                let mut count = 0;

                if x > 0 {
                    if y > 0 {
                        count += count_mine(&counts, x - 1, y - 1);
                    }
                    count += count_mine(&counts, x - 1, y);
                    if y < counts.len() - 1 {
                        count += count_mine(&counts, x - 1, y + 1);
                    }
                }

                if y > 0 {
                    count += count_mine(&counts, x, y - 1);
                }
                if y < counts.len() - 1 {
                    count += count_mine(&counts, x, y + 1);
                }

                if x < counts[y].len() - 1 {
                    if y > 0 {
                        count += count_mine(&counts, x + 1, y - 1);
                    }
                    count += count_mine(&counts, x + 1, y);
                    if y < counts.len() - 1 {
                        count += count_mine(&counts, x + 1, y + 1);
                    }
                }

                assert!(count <= 8);

                counts[y][x] = if count == 0 {
                    ' '
                } else {
                    (count + b'0') as char
                };
            }
        }
    }

    counted_to_string(counts)
}

fn count_mine(counts: &[Vec<char>], x: usize, y: usize) -> u8 {
    if let Some(row) = counts.get(y) {
        if let Some(cell) = row.get(x) {
            return if *cell == '*' { 1 } else { 0 };
        }
    }
    0
}

fn counted_to_string(mine_counts: Vec<Vec<char>>) -> Vec<String> {
    let mut annotated_rows: Vec<String> = Vec::new();

    for row_counts in mine_counts {
        let mut annotated_row = String::new();
        for count in row_counts {
            annotated_row.push(count);
        }
        annotated_rows.push(annotated_row);
    }

    annotated_rows
}
