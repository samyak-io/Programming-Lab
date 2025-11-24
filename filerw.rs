// Program that reads a file, and produces the frequency table
// for exam: #### 2. Largest co-prime subset sum
/* f(10) = 30. {1,5,7,8,9}

Implement it in rust
Tips: look at methods, understand them, then write it from first principles without looking at it again
*/
//rile r/w with freq
//twin primes
//tossing coins 
//logistic map
//cpu temp

use std::fs;
use std::io;

fn main() -> io::Result <()> {
    let file_path = "test_file.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut freq = [0usize; 26];

    for byte in contents.bytes(){
        // convert to lowercase
        let b = byte.to_ascii_lowercase();

        if b.is_ascii_lowercase() {
            freq[(b - b'a') as usize] += 1;
        }
    }

    for i in 0..26 {
        let letter = (b'a' + i as u8) as char;
        println!("{}: {}", letter, freq[i]);
    }

    Ok(())
}