use std::io::{self, BufRead};
use std::env;

fn read_float() -> f64 {
    let mut line = String::new();
    let stdin = io::stdin();
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

fn check_price(args: &Vec<String>) -> f64 {
    match args[args.len()-1].trim().parse() {
        Ok(float) => return float,
        Err(..) => {
            println!("You did not provide a valid price!");
            println!("Please input a total price:");
            return read_float();
        },
    }
}

struct Arzneimittel {
    total_price: f64,
}

impl Arzneimittel {
    pub fn from_total(total_price: f64) -> Self {
        Self {
            total_price,
        }
    }
    pub fn from_buying_price(price: f64) -> Self {
        Self {
            total_price: (price * 1.03 + 8.35 + 0.21 + 0.2) * 1.19,
        }
    }
    pub fn tax(&self) -> f64 {
        self.total_price * 0.19 / 1.19
    }
    fn pharmacy_buying_price(&self) -> f64 {
        (self.total_price - self.tax() - 0.2 - 0.21 - 8.35) / 1.03
    }
    pub fn pharmacy_profit(&self) -> f64 {
        self.total_price - self.tax() - 0.2 - 0.21 - self.pharmacy_buying_price()
    }
    pub fn supplier_profit(&self) -> f64 {
        self.pharmacy_buying_price() - self.producer_price()
    }
    pub fn producer_price(&self) -> f64 {
        (self.pharmacy_buying_price() - 0.73) / 1.0315
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let am = if args.contains(&String::from("-t")) {
        Arzneimittel::from_total(check_price(&args))
    } else if args.contains(&String::from("-b")) {
        Arzneimittel::from_buying_price(check_price(&args))
    } else {
        Arzneimittel::from_total(check_price(&args))
    };
    println!("Total price: {:>8.2}€", am.total_price);
    println!("State Tax:   {:>8.2}€", am.tax());
    println!("Pharmacy:    {:>8.2}€", am.pharmacy_profit());
    println!("Supplier:    {:>8.2}€", am.supplier_profit());
    println!("Producer:    {:>8.2}€", am.producer_price());
}
