pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let players = vec!["X", "O"];

    for player in &players {
        if diagonals(player, &table) || horizontal(player, &table) || vertical(player, &table) {
            return format!("player {} won", player);
        }
    }

    "tie".to_string()
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut main_diag = true;
    let mut sec_diag = true;

    for i in 0..table.len() {
        if table[i][i] != player {
            main_diag = false;
        }
        if table[i][table.len() - 1 - i] != player {
            sec_diag = false;
        }
    }

    main_diag || sec_diag
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for row in table {
        if row.iter().all(|&cell| cell == player) {
            return true;
        }
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for col in 0..table[0].len() {
        let mut column_match = true;
        for row in table {
            if row[col] != player {
                column_match = false;
                break;
            }
        }
        if column_match {
            return true;
        }
    }
    false
}
