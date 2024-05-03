fn main() {
    example(vec![0, 1, 0, 3, 12]);
    example(vec![0]);
    example(vec![0, 0, 1]);
}

fn example(mut vec: Vec<i32>) {
    println!();
    for x in vec.iter() {
        print!("{} ", x)
    }

    Solution::move_zeroes(&mut vec);
    println!();

    for x in vec.iter() {
        print!("{} ", x)
    }
    println!();
}

struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut count: i32 = 0;
        nums.retain(|n| {
            if *n == 0 {
                count += 1;
                return false;
            }
            return true;
        });
        for _ in 0..count {
            nums.push(0);
        }
    }
}
