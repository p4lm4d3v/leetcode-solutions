pub mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn convert(s: String, rows: i32) -> String {
        if rows == 1 {
            return s;
        }

        let n: usize = s.len();
        let mut matrix: Vec<Vec<char>> = vec![Vec::new(); n.min(rows as usize)];

        let mut zigzag: bool = false;
        let mut row: i32 = 0;

        for c in s.chars() {
            matrix[row as usize].push(c);

            if row == 0 || row == rows - 1 {
                zigzag = !zigzag;
            }

            row += if zigzag { 1 } else { -1 }
        }

        let mut result: Vec<char> = Vec::new();

        for r in matrix {
            for c in r {
                result.push(c);
            }
        }

        result.iter().map(|f| f.to_string()).collect::<String>()
    }
}
