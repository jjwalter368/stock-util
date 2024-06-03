use clap::Parser;
use ureq;
use serde_json::{Result, Value};
use std::{fmt, result};
use std::str::FromStr;


// POSTMAN ureq
pub fn postman_ureq_json(url: &str) -> Value {
    let body: Value = serde_json::from_str(ureq::get(url).call().expect(".call failed").into_string().expect(".into_string failed").as_str()).expect("serde_json::from_str() failed");
    body
}
pub fn postman_ureq_str(url: &str) -> String {
    let body: String = ureq::get(url).call().expect(".call failed").into_string().expect(".into_string failed");
    body
}

// turn this into a struct with an option to update or sum
fn parse_fundamental(source: Value) -> FundamentalData {
    let mut output: FundamentalData = FundamentalData::default();
    if source["PERatio"] != "None" {
        output.pe_ratio = source["PERatio"].as_str().expect("pe_ratio, .as_str fail").parse().expect("pe_ratio, .parse() fail"); // f64::from_str(serde_json::from_value::<String>(source["PERatio"].clone()).unwrap().as_str()).unwrap() also works
    }
    if source["ForwardPE"] != "None" {
        output.forward_pe_ratio = source["ForwardPE"].as_str().expect("forward_pe_ratio, .as_str fail").parse().expect("forward_pe_ratio, .parse() fail");
    }
    if source["TrailingPE"] != "None" {
        output.trailing_pe_ratio = source["TrailingPE"].as_str().expect("trailing_pe_ratio, .as_str fail").parse().expect("trailingpe_ratio, .parse() fail");
    }
    if source["PriceToBookRatio"] != "None" {
        output.pb_ratio = source["PriceToBookRatio"].as_str().expect("pb_ratio, .as_str fail").parse().expect("pb_ratio, .parse() fail");
    }
    if source["PEGRatio"] != "None" {
        output.peg_ratio = source["PEGRatio"].as_str().expect("peg_ratio, .as_str fail").parse().expect("peg_ratio, .parse() fail");
    }
    if source["DividendYield"] != "None" {
        output.dividend_yield = source["DividendYield"].as_str().expect("dividend_yield, .as_str fail").parse().expect("dividend_yield, .parse() fail");
    }
    if source["AnalystTargetPrice"] != "None" {
        output.analyst_target_price = source["AnalystTargetPrice"].as_str().expect("analyst_target_price, .as_str fail").parse().expect("analyst_target_price, .parse() fail");
    }
    if source["200DayMovingAverage"] != "None" {
        output.twohundred_ma = source["200DayMovingAverage"].as_str().expect("twohundred_ma, .as_str fail").parse().expect("twohundred_ma, .parse() fail");
    }
    if source["50DayMovingAverage"] != "None" {
        output.fifty_ma = source["50DayMovingAverage"].as_str().expect("fifty_ma, .as_str fail").parse().expect("fifty_ma, .parse() fail");
    }
    output
}

struct Stock {
    ticker: String,
    stock_data: StockData,
}
impl Default for Stock {
    fn default() -> Stock {
        let output: Stock = Stock {
            ticker: String::from("TEST"),
            stock_data: StockData::default()
        };
        output
    }
}

impl fmt::Display for Stock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Ticker: \n\n{}\n\n{}", self.ticker, self.stock_data )
    } 
}
impl Stock {
    fn update(&mut self, ticker: String) {
        self.ticker = ticker.clone();
        self.stock_data.update(ticker);
    }
}

// details of the Stock that gets returned, possiblt will turn it into a class
struct StockData {
    price_data: PriceData,
    fundamental_data: FundamentalData,
}
impl fmt::Display for StockData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Price Data: \n{}\n\nFundamental Data: \n\n{}", self.price_data.price, self.fundamental_data )
    }
}
impl Default for StockData {
    fn default() -> StockData {
        let output: StockData = StockData {
            price_data: PriceData::default(),
            fundamental_data: FundamentalData::default()
        };
        output
    }
}
impl StockData {
    fn update(&mut self, ticker: String) {
        self.price_data.update(ticker.clone());
        self.fundamental_data.update(ticker);
    }
}

struct FundamentalData {
    pe_ratio: f64,
    forward_pe_ratio: f64,
    trailing_pe_ratio: f64,
    pb_ratio: f64,
    peg_ratio: f64,
    dividend_yield: f64,
    analyst_target_price: f64,
    twohundred_ma: f64,
    fifty_ma: f64,
}

impl fmt::Display for FundamentalData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "P/E: {}\nForward P/E: {}\nTrailing P/E: {}\nP/B: {}\nPEG: {}\nDividend Yield: {}\nAnalyst Target Price: {}\n200 MA: {}\n50 MA: {}", self.pe_ratio, self.forward_pe_ratio, self.trailing_pe_ratio, self.pb_ratio, self.peg_ratio, self.dividend_yield, self.analyst_target_price, self.twohundred_ma, self.fifty_ma )
    } 
}

impl Default for FundamentalData {
    fn default() -> FundamentalData {
        let output: FundamentalData = FundamentalData {
            pe_ratio: 0.00,
            forward_pe_ratio: 0.00,
            trailing_pe_ratio: 0.00,
            pb_ratio: 0.00,
            peg_ratio: 0.00,
            dividend_yield: 0.00,
            analyst_target_price: 0.00,
            twohundred_ma: 0.00,
            fifty_ma: 0.00,
        };
        output
    }
}
impl FundamentalData {
    fn update(&mut self, ticker: String) {
        let fundamental_data: Value;
        let fundamental: String = format!("https://www.alphavantage.co/query?function=OVERVIEW&symbol={}&apikey=PSEKBJXKRKARVUTR", ticker);
        fundamental_data = postman_ureq_json(fundamental.as_str());
        *self = parse_fundamental(fundamental_data);
    }
}

struct PriceData {
    price: f64,
}
impl Default for PriceData {
    fn default() -> PriceData {
        let output: PriceData = PriceData {
            price: 0.00
        };
        output
    }
}
impl PriceData {
    fn update(&mut self, ticker: String) {
        let price_data: Value;
        let price: String = format!("https://query1.finance.yahoo.com/v8/finance/chart/{}?region=US&lang=en-US&includePrePost=false&interval=2m&useYfid=true&range=1d&corsDomain=finance.yahoo.com&.tsrc=finance", ticker);
        price_data = postman_ureq_json(price.as_str());
        let output: PriceData =  PriceData {
            price: price_data["chart"]["result"][0]["meta"]["regularMarketPrice"].clone().as_f64().expect(".as_f64 on 146 failed")
        };
        *self = output;
    }
}

// stock-api
fn string_single_analysis(ticker: String) -> Stock {
    let mut output = Stock::default();
    output.ticker = ticker.clone();
    output.update(ticker);
    output
}


//CLAP
#[derive(Parser)]
struct Cli {
    arg1: String,
    arg2: String,
}

//fn cli_interpret(args: &Cli) {
//    //use match case function to do different functions
//    let analysis: String = String::from("analysis");
//}




// TESTING
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn apple_captive() {
        let result = postman_ureq_str("https://captive.apple.com/");
        assert_eq!(result, "<HTML><HEAD><TITLE>Success</TITLE></HEAD><BODY>Success</BODY></HTML>\n");
    }
}
fn main() {
    let args = Cli::parse();
    println!("args.arg1: {:?}, args.arg2: {:?}", args.arg1, args.arg2);

    //cli_interpret(&args);

    if args.arg1 == String::from("analysis") {
        let query_response: Stock = string_single_analysis(args.arg2);
        println!("{}", query_response);
    }
    else {
        println!("Unfortunatley, we have not created any other features than cargo run -- analysis <arg2>");
    }

}
