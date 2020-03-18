extern crate minreq;
use std::result::Result;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;


type Out<T> =  Result<T, Box<dyn std::error::Error>>;

fn download(url :&str) -> Out<Vec<u8>>{
    
    let response = minreq::get(url).send()?;
    Ok(response.into_bytes())
}


fn main() -> Out<()> {
    if let Ok(tmp_bytes) =  download("https://115.236.8.152:50443/dr/kcpee-min/-/raw/master/index.list"){
        let index = match str::from_utf8(&tmp_bytes){
            Ok(s) => s.split('\n').collect(),
            Err(_) => vec![]
        };
        for i in index{
            println!("Down {}", i)
        }
    }else{
        println!("Err : with index" )
    };
    
    
    // sync post request of some json.
    Ok(())
}
