use clap::Parser;
use ureq;

#[derive(Parser)]
struct Cli {
    arg1: String,
    arg2: String,
}

fn cli_interpret(args: &Cli) -> String {
    //use match case function to do different functions
    let analysis: String = String::from("analysis");
    if args.arg1 == analysis {
        println!("Analysis");
    }
    analysis
}

fn postman(url: &str) -> String {
    // placeholder for adding in the ureq module, will do tommorow   
    let data: String = ureq::get(url)
        .set("Example-Header", "header value")
        .call().expect(".call failed")
        .into_string().expect(".call failed");
    data
}


fn main() {
    let args = Cli::parse();
    let query: String;
    let base_url: String =  String::from("http://127.0.0.1:8080/");
    println!("args.arg1: {:?}, args.arg2: {:?}", args.arg1, args.arg2);
    cli_interpret(&args);

    query = format!("{}{}", base_url, args.arg2);

    let query_response: String = postman(query.as_str());
    println!("{}", query_response.as_str());
}












