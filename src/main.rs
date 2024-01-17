use std::io::{self, BufRead};
use std::env;

fn read_price() -> f64 {
    let mut line = String::new();
    let stdin = io::stdin();
    println!("Please input a total price:");
    match stdin.lock().read_line(&mut line) {
        Err(error) => panic!("A problem ocurred with your input! {:?}", error),
        Ok(..) => {
            match line.trim().parse() {
                Err(error) => panic!("You did not input a valid price! {:?}", error),
                Ok(float) => return float,
            };
        },
    };
}

fn get_price(args: Vec<String>) -> f64 {
    if args.len() > 1 {
        match args[1].trim().parse() {
            Err(error) => panic!("You did not input a valid price! {:?}", error),
            Ok(float) => return float,
        }
    }
    return read_price();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let price: f64 = get_price(args);
    let mwst = price - price / 1.19;
    let aep = (price - mwst - 0.2 - 0.21 - 8.35) / 1.03;
    let apo = price - mwst - 0.2 - 0.21 - aep;
    let apu = (aep - 0.73) / 1.0315;
    let gh = aep - apu;
    println!(
        "Total price: {price:>8.2}€\n\
        State Tax:   {mwst:>8.2}€\n\
        Pharmacy:    {apo:>8.2}€\n\
        Supplier:    {gh:>8.2}€\n\
        Producer:    {apu:>8.2}€");
}
