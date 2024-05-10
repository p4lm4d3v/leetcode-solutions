fn main() {
    println!("Hello, world!");
}

struct Player {
    score: i32,
    power: i32,
    tokens: Vec<i32>,
}
impl Player {
    fn new(score: i32, power: i32, tokens: Vec<i32>) -> Self {
        Self {
            score,
            power,
            tokens,
        }
    }
    fn max_score(&self) -> i32 {
        todo!()
    }
}

struct Solution {}
impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        let player = Player::new(0, power, tokens);
        player.max_score()
    }
}

#[cfg(test)]
mod bag_of_tokens_score {
    #[test]
    fn bag_of_tokens_score1() {}
}
