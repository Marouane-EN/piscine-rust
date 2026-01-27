pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let players = "OX";
    for player in players.chars() {
        if diagonals(player, table) {
            return format!("player {player} won");
        }
        if horizontal(player, table) {
            return format!("player {player} won");
        }
        if vertical(player, table) {
            return format!("player {player} won");
        }
    }
    "tie".to_owned()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut count = loop {
        if table[x][y] == player {
            if x == 2 {
                break 2;
            }
            x += 1;
            y += 1;
        } else {
            break 0;
        }
    };
    if count == 2 {
        return count == 2;
    }
    x = 0;
    y = 2;
    count = loop {
        if table[x][y] == player {
            if x == 2 {
                break 2;
            }
            x += 1;
            y -= 1;
        } else {
            break 0;
        }
    };

    count == 2
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    let mut count = 0;
    'inner: for row in table {
        for col in row {
            if col == player {
                count += 1;
            }
        }
        if count == 3 {
            break 'inner;
        }
        count = 0;
    }
    count == 3
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    let mut count = 0;
    'inner: for i in 0..3 {
        for row in 0..table.len() {
            if table[row][i] == player {
                count += 1;
            }
        }
        if count == 3 {
            break 'inner;
        }
        count = 0;
    }
    count == 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            tic_tac_toe([
                ['O', 'X', 'O'],
                ['O', 'P', 'X'],
                ['X', '#', 'X'],
            ]),
            "tie"
        );
    }

    #[test]
    fn it_works1() {
        assert_eq!(
            tic_tac_toe([
                ['X', 'O', 'O'],
                ['X', 'O', 'O'],
                ['#', 'O', 'X'],
            ]),
            "player O won"
        );
    }
    #[test]
    fn it_works2() {
        assert_eq!(
            tic_tac_toe([
                ['O', 'O', 'X'],
                ['O', 'X', 'O'],
                ['X', '#', 'X'],
            ]),
            "player X won"
        );
    }
}
