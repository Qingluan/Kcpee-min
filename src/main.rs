use lazy_static::lazy_static;
use std::env;
use std::fs::read_to_string;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::result::Result;
use std::str;


type Out<T> = Result<T, Box<dyn std::error::Error>>;
const BASE: &'static str = "https://115.236.8.152:50443/dr/kcpee-min/-/raw/master/";
lazy_static!{
    static ref TEMP_DIR: PathBuf = env::temp_dir();
}


fn download<F>(url: &str, dst: &str, after: Option<F>) -> Out<()>
where
    F: Fn(&str) + Send,
{
    // let mut dst_uri = format!("C:\\tmp\\{}", dst);
    let mut dst_path = TEMP_DIR.clone();
    dst_path.push(dst);
    let dst_uri = dst_path.to_str().expect("no").to_string();
    let _ = if cfg!(target_os = "windows") {
        let wincmd = format!(
            "$client = new-object System.Net.WebClient;$client.DownloadFile('{}','{}') ;",
            url, dst_uri
        );
        let mut process = Command::new("powershell")
            .args(&["-Command", "-"])
            .stdin(Stdio::piped())
            .spawn()?;
        let stdin = process.stdin.as_mut().expect("pipe failure");
        stdin
            .write_all(wincmd.as_bytes())
            .expect("ps downlaod error");
    } else {
        // dst_uri = format!("/tmp/{}", dst);
        let cmd = format!("curl -ksSl '{}' -o '{}' ;", url, dst_uri);
        println!("{}", cmd);
        Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("may wget error ");
    };
    if let Some(after_fun) = after {
        after_fun(&dst_uri);
        Ok(())
    } else {
        Ok(())
    }
}

fn after_dosome(some: &str) {
    println!("{} [ok]", some)
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
                        println!("found new file to donwload: {}", each_name);
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

fn main() -> Out<()> {
    download(
        &format!("{}{}", BASE, "index.list"),
        "index.list",
        Some(read_index),
    )
    // sync post request of some json.
    // Ok(())
}
