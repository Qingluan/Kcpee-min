
use std::result::Result;
use std::io::Write;
use std::str;
use std::process::{Command, Stdio};
use std::fs::read_to_string;


type Out<T> =  Result<T, Box<dyn std::error::Error>>;



fn download<F>(url :&str, dst :&str,after : Option<F> )-> Out<()>
where F: FnOnce(&str)  +Send
{
    
    let mut dst_uri  = format!("C:\\tmp\\{}" , dst); 
    let _ = if cfg!(target_os = "windows") {
        
        let wincmd = format!("client = new-object System.Net.WebClient;client.DownloadFile('{}','{}');",url, dst_uri );
        let mut process = Command::new("powershell")
                      .args(&["-Command", "-"])
                      .stdin(Stdio::piped())
                      .spawn()?;
        let stdin = process.stdin.as_mut().expect("pipe failure");
        stdin.write_all(wincmd.as_bytes()).expect("ps downlaod error");
    } else {
        dst_uri = format!("/tmp/{}", dst);
        let cmd = format!("wget -c '{}' -O '/tmp/{}');",url, dst_uri );
        Command::new("bash")
                .arg("-c")
                .arg(cmd)
                .output().expect("may wget error ");
    };
    if let Some(after_fun) = after{
        after_fun(&dst_uri);
        Ok(())
    }else{
        Ok(())
    }
}
fn read_index (filepath: &str) {
    let sep = if cfg!(target_os="windows"){
        "\r\n"
    }else{
        "\n"
    };
    
    let _ = match read_to_string(filepath){
        Ok(ss) => {
            let _ = ss.split(sep).collect::<Vec<&str>>().iter().map(|&each_name| {
                println!("found new file to donwload: {}", each_name)
            }).collect::<Vec<_>>();
        },
        Err(e ) => println!("{}",e)
    };
    
}

fn main() -> Out<()> {
   
    download("https://115.236.8.152:50443/dr/kcpee-min/-/raw/master/index.list", "index.list", Some(read_index) )
    // sync post request of some json.
    // Ok(())
}
