
pub fn to_grid(code: &str) -> Vec<Vec<char>> {
    let mut arr = vec![];
    for row in code.split("\n") {
        let mut col = vec![];
        for character in row.chars() {
            if character == ' ' { continue; };
            col.push(character);
        }
        if col.len() == 0 { continue; };
        arr.push(col);
    };
    return arr;
}