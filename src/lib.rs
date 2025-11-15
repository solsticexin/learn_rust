use std::fs;
use std::error::Error;
pub struct Config{
    pub query:String,
    pub file_path:String,
}

#[warn(dead_code)]
pub fn parse_config(args:&[String])->Config{
    let query=args[1].clone();
    let file_path=args[2].clone();
    Config{
        query,
        file_path,
    }
}
//将parse_config转换为config结构体关联的函数
impl Config {
    //不要采用new,这是rust的约定熟成,因为许多程序员希望 new 函数永远不会失败.
    #[warn(dead_code)]
    pub fn new(args:&[String])->Self{
        if args.len() < 3{ panic!("no enough arguments")}
        Self { query:args[1].clone(), file_path: args[2].clone() }
    }
    pub fn build(args:&[String])->Result<Self,&'static str>{
        if args.len() < 3{ return Err("no enough arguments")}
        Ok(Self { query:args[1].clone(), file_path: args[2].clone() })
    }
}
pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents=fs::read_to_string(config.file_path)?;
    // .expect("Should have been able to read the file");
    println!("With test:\n{contents}");
    Ok(())
}