pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let player_o = "O";
    let player_x = "X";

    if horizontal(player_o, &table) || vertical(player_o, &table) || diagonals(player_o, &table) {
        return "player O won".to_string();
    }
    if horizontal(player_x, &table) || vertical(player_x, &table) || diagonals(player_x, &table) {
        return "player X won".to_string();
    }

    "tie".to_string()
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for row in table.iter() {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if (table[0][0] == player && table[1][0] == player && table[2][0] == player) ||
    (table[0][1] == player && table[1][1] == player && table[2][1] == player) ||
    (table[0][2] == player && table[1][2] == player && table[2][2] == player) {
     return true;
 }
   return false;
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if (table[0][0] == player && table[1][1] == player && table[2][2] == player) ||
       (table[0][2] == player && table[1][1] == player && table[2][0] == player) {
        return true;
    }
    return false;
}
