extern crate ureq;
extern crate csv;

use std::io;

use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Hash, Eq, PartialEq, Debug, Ord, PartialOrd, Deserialize)]
struct CovidData {
    date : String,
    state : String,
    cases : u32,
    deaths : u32,
}

fn parse_csv<D: DeserializeOwned, R: io::Read>(rdr: R) -> csv::Result<Vec<D>> {
    csv::Reader::from_reader(rdr).into_deserialize().collect()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // sync post request of some json.
    let req = ureq::get("https://raw.githubusercontent.com/nytimes/covid-19-data/master/us-states.csv")
        .call()
        .into_string()?;
        
    let strreq = req.as_str();
    let covid_data : Vec<CovidData> = parse_csv(strreq.as_bytes())?;
    let filtered : Vec<CovidData> = covid_data
        .into_iter()
        .filter(|d| d.state == "Colorado")
        .collect();

    println!("{:?}", &filtered);
    

    Ok(())
    // .ok() tells if response is 200-299.
    
}