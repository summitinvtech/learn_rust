extern crate ureq;
extern crate csv;

use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CovidData {
    date : String,
    state : String,
    cases : u32,
    deaths : u32,
}
// fn getdata() -> Result<Vec<CovidData>, Box<dyn Error>> {
//     let mut rdr = csv::Reader::from_reader(io::stdin());
//     for result in rdr.deserialize() {
//         // Notice that we need to provide a type hint for automatic
//         // deserialization.
//         let record: CovidData = result?;
//         println!("{:?}", record);
//         record
//     }
//     Ok(())
// }
fn main() {

    // sync post request of some json.
    let req = ureq::get("https://raw.githubusercontent.com/nytimes/covid-19-data/master/us-states.csv")
        .call()
        .into_string()
        .unwrap();
    let mut csv = csv::Reader::from_reader(req.as_bytes());
    
    let results = {
        for result in csv.deserialize(){
            let record : CovidData = result.unwrap();
            println!("{:?}", record);
        };
    };
    println!("{:?}", results);
    
    // .ok() tells if response is 200-299.
    
}