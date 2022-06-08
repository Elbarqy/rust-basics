extern crate phrases;

fn main() {
    println!(
        "this is how hello is said in English {}",
        phrases::english::greetings::hello()
    );

    println!(
        "this is how hello is said in Japanese {}",
        phrases::english::greetings::hello()
    );
}
