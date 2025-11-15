use std::env;
use std::process;
use learn_rust::Config;

fn main(){
    //读取命令行,第一个被自身程序所占领,所以送下标1开始.
    let args:Vec<String>=env::args().collect();
    // dbg!(args);
    // let query=&args[1];
    // let file_path=&args[2];
    // let config=parse_config(&args);
    let config=Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments {err}");
        process::exit(1);
    });
    println!("query:{:?},file_path:{:?}",config.query,config.file_path);
    // let contents=fs::read_to_string(config.file_path).expect("no search file");
    // println!("with test:\n{contents}");
    // run(config);
    if let Err(e) = learn_rust::run(config) {
        println!("application error:{e}");
        process::exit(1);
    }
}
