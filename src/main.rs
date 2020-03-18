extern crate minreq;
use std::result::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // sync post request of some json.
    let response = minreq::get("http://httpbin.org/ip").send()?;
    println!("{}",response.as_str()?);
    Ok(())
}
