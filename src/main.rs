use std::io::stdin;

fn main() {
    let mut sudoku = [[0; 9]; 9];
    for i in 0..9 {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        for j in 0..9 {
            let c = line.chars().nth(j).unwrap();
            sudoku[i][j] = if c == '*' { 0 } else { c as i32 - '0' as i32 }
        }
    }

    solve(&mut sudoku, 0, 0);
    println!();
    for i in 0..9 {
        for j in 0..9 {
            print!("{}", std::char::from_digit(sudoku[i][j] as u32, 10).unwrap());
        }
        println!();
    }
}

fn solve(sudoku: &mut [[i32; 9]; 9], i: usize, j: usize) -> bool {
    if i == 9 || j == 9 { return true; }
    if sudoku[i][j] != 0 { return solve(sudoku, i + (j + 1) / 9, (j + 1) % 9); }

    for n in 1..=9 {
        sudoku[i][j] = n;

        if check(sudoku) {
            if i == 8 && j == 8 { return true; }
        } else {
            continue;
        }

        if solve(sudoku, i + (j + 1) / 9, (j + 1) % 9) {
            return true;
        }
    }

    sudoku[i][j] = 0;

    false
}

fn check(sudoku: &[[i32; 9]; 9]) -> bool {
    for i in 0..9 {
        let mut col = [false; 10];
        let mut row = [false; 10];
        let mut chunk = [false; 10];
        for j in 0..9 {
            let n = sudoku[j][i];
            if n != 0 {
                if col[n as usize] { return false; }
                col[n as usize] = true;
            }

            let n = sudoku[i][j];
            if n != 0 {
                if row[n as usize] { return false; }
                row[n as usize] = true;
            }

            let n = sudoku[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3];
            if n != 0 {
                if chunk[n as usize] { return false; }
                chunk[n as usize] = true;
            }
        }
    }

    true
}
