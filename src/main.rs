use clap::Parser;
use ureq;

fn main() {
    let args = Cli::parse();
    let mut query: String;
    println!("args.arg1: {:?}, args.arg2: {:?}", args.arg1, args.arg2);
    cli_interpret(args);

    //make sure to change the url to fit the "rocket" url
    query = format!("{}{}", String::from("http://127.0.0.1:8080/"), args.arg2);

    let query_response = postman(query);
    println!(query_response);
}

#[derive(Parser)]
struct Cli {
    arg1: String,
    arg2: String,
}

fn cli_interpret(args: Cli) -> String {
    //use match case function to do different functions
    let analysis: String = String::from("analysis");
    match args.arg1 {
        _analysis => println!("Analysis"),
    }
    return analysis;
}

fn postman(url: String) -> JsonValue {
    // placeholder for adding in the ureq module, will do tommorow   
    return OK(());
}
