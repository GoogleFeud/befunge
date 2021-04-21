
const MIN_X: usize = 25;
const MIN_Y: usize = 80;


pub fn to_grid(code: &str) -> Vec<Vec<char>> {
    let mut arr = vec![];
    let rows: Vec<&str> = code.split('\n').collect();
    for index in 0..=MIN_X {
        let mut col = vec![];
        if rows.len() > index {
            for mut character in rows[index].chars() {
                if character == '\r' { character = ' '; }
                col.push(character);
            }
            while col.len() < MIN_Y {
                col.push(' ');
            }
        }
        else {
            for _ in col.len()..MIN_Y {
                col.push(' ');
            }
        }
        arr.push(col);
    }
    arr
}