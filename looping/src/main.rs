use std::io;

fn main() {
    let mut count = 1;
    let mut s = String::new();
    loop {
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );

        let _ = io::stdin().read_line(&mut s);

        if s.trim() == "The letter e" {
            println!("Number of trials: {count}");
            break;
        }
        count += 1;
        s.clear();
    }
}
