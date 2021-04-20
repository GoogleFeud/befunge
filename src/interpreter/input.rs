
pub fn to_grid(code: &str) -> Vec<Vec<char>> {
    let mut arr = vec![];
    for row in code.split("\n") {
        let mut col = vec![];
        for character in row.chars() {
            col.push(character);
        }
        arr.push(col);
    };
    return arr;
}