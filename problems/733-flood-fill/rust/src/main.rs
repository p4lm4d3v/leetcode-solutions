mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let m: i32 = image.len() as i32;
        let n: i32 = image[0].len() as i32;
        let target: i32 = image[sr as usize][sc as usize];

        if color == target {
            return image;
        }

        fn dfs(i: i32, j: i32, m: i32, n: i32, target: i32, color: i32, image: &mut Vec<Vec<i32>>) {
            if i < 0 || i >= m || j < 0 || j >= n || image[i as usize][j as usize] != target {
                return;
            }

            image[i as usize][j as usize] = color;

            dfs(i - 1, j, m, n, target, color, image);
            dfs(i + 1, j, m, n, target, color, image);
            dfs(i, j - 1, m, n, target, color, image);
            dfs(i, j + 1, m, n, target, color, image);
        }

        dfs(sr, sc, m, n, target, color, &mut image);
        image
    }
}
