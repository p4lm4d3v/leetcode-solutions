mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn count_students(mut students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        let mut count: i32 = 0;

        while students.len() != 0 && sandwiches.len() != 0 {
            let student: i32 = students[0];
            let sandwich: i32 = sandwiches[0];

            let before: String = students.iter().map(|f| f.to_string()).collect();

            if student != sandwich {
                let first: i32 = students[0];
                students = students[1..].to_vec();
                students.push(first);
            } else {
                students = students[1..].to_vec();
                sandwiches = sandwiches[1..].to_vec();
            }

            let after: String = students.iter().map(|f| f.to_string()).collect();

            if before == after {
                count = before.len() as i32;
                break;
            }
        }

        count
    }
}
