use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum, Default)]
enum InputType {
    /// total price of the pharmaceutical
    #[default]
    Total,
    /// buying price of the pharmaceutical
    Buying,
}

#[derive(Debug, Clone, clap::ValueEnum, Default)]
enum OutputType {
    /// full price composition
    #[default]
    Full,
    /// part of the price going to the pharmacy
    Pharmacy,
    /// part of the price going to the supplier
    Supplier,
    /// part of the price going to the state as VAT
    Tax,
    /// part of the price going to the producer
    Producer,
}

#[derive(Parser, Debug)]
#[command(about, version, author)]
struct Cli {
    #[arg(short = 'i', long, default_value_t, value_enum)]
    input_type: InputType,
    #[arg(short = 'o', long, default_value_t, value_enum)]
    output_type: OutputType,
    #[arg(value_name = "PRICE")]
    price: f64,
}

struct Arzneimittel {
    total_price: f64,
}

impl Arzneimittel {
    fn from(input_type: InputType, price: f64) -> Self {
        match input_type {
            InputType::Total => Self {
                total_price: price,
            },
            InputType::Buying => Self {
                total_price: (price * 1.03 + 8.35 + 0.21 + 0.2) * 1.19,
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
    let am = Arzneimittel::from(args.input_type, args.price);
    match args.output_type {
        OutputType::Full => {
            println!("Total price: {:>8.2}€", am.total_price);
            println!("State Tax:   {:>8.2}€", am.tax());
            println!("Pharmacy:    {:>8.2}€", am.pharmacy_profit());
            println!("Supplier:    {:>8.2}€", am.supplier_profit());
            println!("Producer:    {:>8.2}€", am.producer_price());
        },
        OutputType::Pharmacy => {
            println!("{:.2}", am.pharmacy_profit());
        },
        OutputType::Supplier => {
            println!("{:.2}", am.supplier_profit());
        },
        OutputType::Tax => {
            println!("{:.2}", am.tax());
        },
        OutputType::Producer => {
            println!("{:.2}", am.producer_price());
        },
    }
}
