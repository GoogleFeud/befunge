
pub fn to_grid(code: &str) -> Vec<Vec<char>> {
    let mut arr = vec![];
    for row in code.split("\n") {
        let mut col = vec![];
        for mut character in row.chars() {
            if character == '\r'{ character = ' '; }
            col.push(character);
        }
        arr.push(col);
    };
    return arr;
}