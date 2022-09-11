use std::io;
use std::time::{Instant};

#[tokio::main]
async fn main() {
    let mut round = 0;
    let mut score: i32 = 0;
    let mut recorded_times = Vec::new();
    while round < 100 {
        let word = match get_random_word().await {
            Ok(w) => w,
            Err(_) => panic!("Failed to fetch word from random word API"),
        };

        println!("Write the following word as fast as possible: {:?}", word);
        let start = Instant::now();
        let mut word_input = String::new();
        io::stdin()
            .read_line(&mut word_input)
            .expect("Failed to read line");
        let time_elapsed = start.elapsed().as_secs();
        recorded_times.push(time_elapsed);
        
        if word == word_input.as_str().trim() {
            score += 1;
            println!("✔ you made it! 💅 | time to write: {:#?}s | score = {:#?}", time_elapsed, score);
        } else {
            println!("❌ oh no...you failed 🤕, your average writing time: {:#?}s | exiting now...", recorded_times.iter().sum::<u64>() / recorded_times.len() as u64 );
            break;
        }
        round += 1;
    }
}

async fn get_random_word() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://random-word-api.herokuapp.com/word")
        .await?
        .json::<[String; 1]>()
        .await?;
    let word = resp[0].clone();
    Ok(word)
}
