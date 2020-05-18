extern crate ureq;

fn main() {

    // sync post request of some json.
    let req = ureq::get("https://raw.githubusercontent.com/nytimes/covid-19-data/master/us-states.csv")
        .call()
        .into_string();
    
        
    println!("{:?}", req);
    // .ok() tells if response is 200-299.
    
}