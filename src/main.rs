// Trivia Time by danrfq

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

use std::string::String;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::io;
use itertools::Itertools;

fn escape_html(raw: &json::JsonValue) -> String {
    htmlescape::decode_html(&raw.to_string()).expect("Somehow, there's an unexpected error while running the program. Please try again.")
}

fn main() -> Result<()> {
    let raw_data = reqwest::blocking::get("https://opentdb.com/api.php?amount=1&type=multiple")?.text()?;
    let data = &json::parse(&raw_data).unwrap()["results"][0];
    let question = escape_html(&data["question"]);
    let category = escape_html(&data["category"]);
    let correct = escape_html(&data["correct_answer"]);
    let letters = vec![String::from("A"), String::from("B"), String::from("C"), String::from("D")];

    let mut answers = Vec::new();
    answers.push(escape_html(&data["correct_answer"]));
    answers.push(escape_html(&data["incorrect_answers"][0]));
    answers.push(escape_html(&data["incorrect_answers"][1]));
    answers.push(escape_html(&data["incorrect_answers"][2]));

    let mut rng = thread_rng();
    answers.shuffle(&mut rng);

    println!("Trivia Time - {}", category);
    println!("Question: {}", question);

    let mut final_: HashMap<&str, &str> = HashMap::new();
    final_.insert("A", &answers[0]);
    final_.insert("B", &answers[1]);
    final_.insert("C", &answers[2]);
    final_.insert("D", &answers[3]);

    let mut correct_letter = "".to_string();

    for letter in final_.keys().sorted() {
        println!("{}. {}", letter, final_[letter]);
    }

    println!("Type the letter [A, B, C or D] of your answer below! (case insensitive)");

    let mut ans = String::new();
    io::stdin()
    .read_line(&mut ans)
    .expect("Failed to read line. Please try again later.");

    let answer = ans.trim().to_string().to_ascii_uppercase();
    let final_answer: &str = &answer;

    if letters.contains(&answer) {
        if final_[final_answer] == correct {
            println!("You're right!");
        } else {
            for (letter, answer) in final_ {
                if answer == correct {
                    correct_letter.push_str(letter);
                }
            }
            println!("You're wrong. The correct answer is {}. {}.", correct_letter, correct)
        }
    } else {
        println!("Bruh, you were supposed to type in either A, B, C, or D. Go back to kindergarten.");
    }
    Ok(())
}
