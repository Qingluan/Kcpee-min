#![windows_subsystem = "windows" ]
use lazy_static::lazy_static;
use std::env;
use std::fs::read_to_string;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::result::Result;
use std::str;

type Out<T> = Result<T, Box<dyn std::error::Error>>;
const BASE: &'static str = "https://raw.githubusercontent.com/Qingluan/Kcpee-min/master/";
const BASE2: &'static str = "https://gitee.com/dark.H/Kcpee-min/raw/master/";

lazy_static! {
    static ref TEMP_DIR: PathBuf = env::temp_dir();
}

fn download<F>(url: &str, dst: &str, after: Option<F>) -> Out<()>
where
    F: Fn(&str) + Send + Copy,
{
    // let mut dst_uri = format!("C:\\tmp\\{}", dst);
    let mut dst_path = TEMP_DIR.clone();
    dst_path.push(dst.trim());
    let dst_uri = dst_path.to_str().expect("no").trim().to_string();
    let _ = if cfg!(target_os = "windows") {
        let wincmd = format!(
            "$client = new-object System.Net.WebClient;$client.DownloadFile(\"{}\",\"{}\");",
            url.trim(), dst_uri
        );
        // println!("Run : {}", wincmd);
        let mut process = Command::new("powershell")
            .args(&["-NoProfile", "-ExecutionPolicy", "unrestricted","-windowstyle","hidden","-Command", "-"])
            .stdin(Stdio::piped())
            .spawn()?;
        let stdin = process.stdin.as_mut().expect("pipe failure");
        stdin
            .write_all(wincmd.as_bytes())
            .expect("ps downlaod error");
        match process.wait() {
            Ok(code) => { 
                if code.code() != Some(0){
                    let f_names = url.split("/").collect::<Vec<_>>();
                    let f_name = f_names.last().unwrap();
                    println!("[err] try route2: {} ",url.trim());
                    download(&format!("{}{}", BASE2, f_name), dst, after)?;
                }else{
                    println!("[ok] downlod file: {} ",dst.trim());
                }
            },
            Err(_) => {
                
            }
        }
    } else {
        // dst_uri = format!("/tmp/{}", dst);
        let cmd = format!("curl -ksSl '{}' -o '{}' ;", url, dst_uri);
        // println!("{}", cmd);
        let c = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("may wget error ");
        if c.status.code() != Some(0){
            let f_names = url.split("/").collect::<Vec<_>>();
            let f_name = f_names.last().unwrap();
            println!("[err] try route2: {} ",url.trim());
            download(&format!("{}{}", BASE2, f_name), dst, after)?;
        }else{
            println!("[ok] downlod file: {} ",dst.trim());
        }
    };
    if let Some(after_fun) = after {
        after_fun(&dst_uri);
        Ok(())
    } else {
        Ok(())
    }
}

fn after_dosome(some: &str) {
    println!("[done] {}", some)
}

fn read_index(filepath: &str) {
    let sep = if cfg!(target_os = "windows") {
        "\r\n"
    } else {
        "\n"
    };

    let _ = match read_to_string(filepath) {
        Ok(ss) => {
            let _ = ss
                .split(sep)
                .collect::<Vec<&str>>()
                .iter()
                .map(|&each_name| {
                    if each_name.trim() != "" {
                        println!("    [+] need to donwload: {}", each_name.trim());
                        let _ = download(
                            &format!("{}{}", BASE, each_name),
                            each_name,
                            Some(after_dosome),
                        );
                    }
                })
                .collect::<Vec<_>>();
        }
        Err(e) => println!("{}", e),
    };
}

fn background(runner :&str) -> Out<()>{
    
    let wincmd = format!("{} f", runner);
    if cfg!( target_os="windows" ){
        let mut process = Command::new("powershell")
            .args(&["-Command", "-"])
            .stdin(Stdio::piped())
            .spawn().expect("some error for initialization powershell");
        let stdin = process.stdin.as_mut().expect("pipe failure");
        stdin
            .write_all(wincmd.clone().as_bytes()).expect("interact ps error!");
    }else{
        Command::new("bash")
            .arg("-c")
            .arg(&wincmd)
            .spawn().expect("linux initializaltion err");
            // process.spawn();
    }

    Ok(())
}

fn main() -> Out<()> {
    let mut run_foreground = false;
    let mut c = 0;
    let mut runner:String = String::from("");
    for arg in env::args(){
        if c ==0 {
            runner = arg.clone();
        }
        if arg == "f"{
            download(
                &format!("{}{}", BASE, "index.list"),
                "index.list",
                Some(read_index),
            )?;
            run_foreground = true;
        }
        c+=1;
    }
    if !run_foreground && &runner != ""{
        println!("ready background!");
        background(&runner).expect("run backgroun err!");
        Ok(())
    }else{
        Ok(())
    }
    
    // sync post request of some json.

}
