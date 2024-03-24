use num::Integer;
use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

// Prompt the user for an i32.
pub fn get_i32(prompt: &str) -> i32 {
    get_int(prompt).expect("Error parsing integer")
}

// Prompt the user for an i64.
pub fn get_i64(prompt: &str) -> i64 {
    get_int(prompt).expect("Error parsing integer")
}

// Prompt the user for an integer
fn get_int<T: Integer + FromStr + Debug>(prompt: &str) -> Result<T, <T as FromStr>::Err> {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    str_value.trim().parse::<T>()
}

pub struct Prng {
    seed: u32,
}

impl Prng {
    pub fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    pub fn new_with_seed(seed: u32) -> Self {
        Self { seed }
    }

    pub fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    pub fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    pub fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    // Return a pseudorandom value in the range [min, max).
    pub fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i32;
    }

    // Return a pseudorandom value in the range [min, max).
    pub fn next_i64(&mut self, min: i64, max: i64) -> i64 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i64;
    }
}

// Make a vector of random i32 values in the range [0 and max).
pub fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    vec
}

// Create string of first num_items items
pub fn make_vec_string<T: std::fmt::Display>(vec: &Vec<T>, num_items: i32) -> String {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    string
}

// Print at most num_items items.
pub fn print_vec<T: std::fmt::Display>(vec: &Vec<T>, num_items: i32) {
    println!("{}", make_vec_string(vec, num_items));
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
