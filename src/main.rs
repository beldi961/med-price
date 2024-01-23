use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum, Default)]
enum InputType {
    /// total price of the pharmaceutical
    #[default]
    Total,
    /// total price of the pharmaceutical
    Buying,
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Cli {
    #[clap(short = 'i', long, default_value_t, value_enum)]
    input_type: InputType,
    #[clap(value_name = "PRICE")]
    price: f64,
}

struct Arzneimittel {
    total_price: f64,
}

enum PriceType {
    Total(f64),
    Buying(f64),
}

impl Arzneimittel {
    fn from(price: PriceType) -> Self {
        match price {
            PriceType::Total(f) => Self {
                total_price: f,
            },
            PriceType::Buying(f) => Self {
                total_price: (f * 1.03 + 8.35 + 0.21 + 0.2) * 1.19,
            },
        }
    }
    fn tax(&self) -> f64 {
        self.total_price * 0.19 / 1.19
    }
    fn pharmacy_buying_price(&self) -> f64 {
        (self.total_price - self.tax() - 0.2 - 0.21 - 8.35) / 1.03
    }
    fn pharmacy_profit(&self) -> f64 {
        self.total_price - self.tax() - 0.2 - 0.21 - self.pharmacy_buying_price()
    }
    fn supplier_profit(&self) -> f64 {
        self.pharmacy_buying_price() - self.producer_price()
    }
    fn producer_price(&self) -> f64 {
        (self.pharmacy_buying_price() - 0.73) / 1.0315
    }
}

fn main() {
    let args = Cli::parse();
    // dbg!(args.input_type);
    let price = match args.input_type {
        InputType::Total => PriceType::Total(args.price),
        InputType::Buying => PriceType::Buying(args.price),
    };
    let am = Arzneimittel::from(price);
    println!("Total price: {:>8.2}€", am.total_price);
    println!("State Tax:   {:>8.2}€", am.tax());
    println!("Pharmacy:    {:>8.2}€", am.pharmacy_profit());
    println!("Supplier:    {:>8.2}€", am.supplier_profit());
    println!("Producer:    {:>8.2}€", am.producer_price());
}
