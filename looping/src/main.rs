use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let correct_answer = "The letter e";

    let mut tries = 0;

    loop {
        println!("{}", riddle);

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input = user_input.trim();
        /*println!("{}",user_input);
        println!("{}", correct_answer);*/
        if user_input == correct_answer {
            tries += 1;
            println!("Number of trials: {}", tries);
            break;
        } else {
            //println!("Sorry, that's incorrect. Please try again.");
            tries += 1;
        }
    }
}
