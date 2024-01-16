use std::io::{self, BufRead};

fn get_price() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    println!("Please input a total price:");
    match stdin.lock().read_line(&mut line) {
        Err(error) => panic!("A problem ocurred with your input! {:?}", error),
        Ok(..) => return line,
    };
}

fn main() {
    let price: f64 = match get_price().trim().parse() {
        Err(error) => panic!("You did not input a valid price! {:?}", error),
        Ok(float) => float,
    };
    println!("Total price: {:>8.2}€", price)
    // print components
}
