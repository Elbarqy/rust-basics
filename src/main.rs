extern crate phrases;

fn main() {
    println!(
        "this is how hello is said in English {}",
        phrases::english::greetings::hello()
    );

    println!(
        "this is how hello is said in Japanese {}",
        phrases::japanese::greetings::hello()
    );

    println!(
        "this is how Good bye is said in English {}",
        phrases::english::farewell::goodbye()
    );

    println!(
        "this is how Good bye is said in Japanese {}",
        phrases::japanese::farewell::goodbye()
    );
}
