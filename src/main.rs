use clap::Parser;
use ureq;
use serde_json::{Result, Value};
use std::{fmt, result};



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
   
    for x in source["timeseries"]["result"].as_array().unwrap() {
        match x[0]["meta"]["type"][0].to_string().as_str() {
            "quarterlyPeRatio" => output.quarterlyPeRatio = x[0]["quarterlyPeRatio"].as_array().unwrap().last().unwrap()["reportedValue"]["raw"].as_f64().unwrap(),
            "quarterlyForwardPeRatio" => output.quarterlyForwardPeRatio = x[0]["quarterlyForwardPeRatio"].as_array().unwrap().last().unwrap()["reportedValue"]["raw"].as_f64().unwrap(),
            "quarterlyPegRatio" => output.quarterlyPegRatio = x[0]["quarterlyPegRatio"].as_array().unwrap().last().unwrap()["reportedValue"]["raw"].as_f64().unwrap(),
            "quarterlyPsRatio" => output.quarterlyPeRatio = x[0]["quarterlyPsRatio"].as_array().unwrap().last().unwrap()["reportedValue"]["raw"].as_f64().unwrap(),
            "quarterlyMarketCap" => output.quarterlyPeRatio = x[0]["quarterlyMarketCap"].as_array().unwrap().last().unwrap()["reportedValue"]["raw"].as_f64().unwrap(),
            "quarterlyEnterprisesValueRevenueRatio" => output.quarterlyEnterprisesValueRevenueRatio = x[0]["quarterlyEnterprisesValueRevenueRatio"].as_array().unwrap().last().unwrap()["reportedValue"]["raw"].as_f64().unwrap(),
            "quarterlyEnterprisesValueEBITDARatio" => output.quarterlyEnterprisesValueEBITDARatio = x[0]["quarterlyEnterprisesValueEBITDARatio"].as_array().unwrap().last().unwrap()["reportedValue"]["raw"].as_f64().unwrap(),
            "quarterlyEnterpriseValue" => output.quarterlyEnterpriseValue = x[0]["quarterlyEnterpriseValue"].as_array().unwrap().last().unwrap()["reportedValue"]["raw"].as_f64().unwrap(),
            "quarterlyPbRatio" => output.quarterlyPbRatio = x[0]["quarterlyPbRatio"].as_array().unwrap().last().unwrap()["reportedValue"]["raw"].as_f64().unwrap(),
            _ => continue,
        }
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
        write!( f, "Ticker: \n{}\nPrice Data: \n{}\nFundamental Data: \n{}", self.ticker, self.stock_data.price_data.price, self.stock_data.fundamental_data.quarterlyPegRatio ) //NEED TO CHANGE THE FUNDAMENTAL DATA
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
        write!( f, "Price Data: \n{}\n Fundamental Data: \n{}", self.price_data.price, self.fundamental_data.quarterlyMarketCap ) //NEED TO CHANGE THE FUNDAMENTAL DATA
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
    quarterlyPeRatio: f64,
    quarterlyForwardPeRatio: f64,
    quarterlyPegRatio: f64,
    quarterlyPsRatio: f64,
    quarterlyMarketCap: f64,
    quarterlyEnterprisesValueRevenueRatio: f64,
    quarterlyEnterprisesValueEBITDARatio: f64,
    quarterlyEnterpriseValue: f64,
    quarterlyPbRatio: f64,
}
impl Default for FundamentalData {
    fn default() -> FundamentalData {
        let output: FundamentalData = FundamentalData {
            quarterlyPeRatio: 0.00,
            quarterlyForwardPeRatio: 0.00,
            quarterlyPegRatio: 0.00,
            quarterlyPsRatio: 0.00,
            quarterlyMarketCap: 0.00,
            quarterlyEnterprisesValueRevenueRatio: 0.00,
            quarterlyEnterprisesValueEBITDARatio: 0.00,
            quarterlyEnterpriseValue: 0.00,
            quarterlyPbRatio: 0.00,
        };
        output
    }
}
impl FundamentalData {
    fn update(&mut self, ticker: String) {
        let fundamental_data: Value;
        let fundamental: String = format!("https://query1.finance.yahoo.com/ws/fundamentals-timeseries/v1/finance/timeseries/{}?merge=false&padTimeSeries=true&period1=493590046&period2=1708996941&type=quarterlyMarketCap%2CtrailingMarketCap%2CquarterlyEnterpriseValue%2CtrailingEnterpriseValue%2CquarterlyPeRatio%2CtrailingPeRatio%2CquarterlyForwardPeRatio%2CtrailingForwardPeRatio%2CquarterlyPegRatio%2CtrailingPegRatio%2CquarterlyPsRatio%2CtrailingPsRatio%2CquarterlyPbRatio%2CtrailingPbRatio%2CquarterlyEnterprisesValueRevenueRatio%2CtrailingEnterprisesValueRevenueRatio%2CquarterlyEnterprisesValueEBITDARatio%2CtrailingEnterprisesValueEBITDARatio&lang=en-US&region=US", ticker);
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
            price: price_data["chart"]["result"][0]["meta"]["regularMarketPrice"].clone().as_f64().expect(".as_f64 on 143 failed")
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
