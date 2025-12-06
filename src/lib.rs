pub mod tree;




use std::{env, fs};
use std::error::Error;
pub struct Config{
    pub query:String,
    pub file_path:String, 
    pub ignore_case:bool,
}

#[warn(dead_code)]
pub fn parse_config(args:&[String])->Config{
    let query=args[1].clone();
    let file_path=args[2].clone();
    Config{
        query,
        file_path,
        ignore_case:true,
    }
}
//将parse_config转换为config结构体关联的函数
impl Config {
    //不要采用new,这是rust的约定熟成,因为许多程序员希望 new 函数永远不会失败.
    #[warn(dead_code)]
    pub fn new(args:&[String])->Self{
        if args.len() < 3{ panic!("no enough arguments")}
        Self { query:args[1].clone(), file_path: args[2].clone(),ignore_case:true }
    }
    pub fn build(args:&[String])->Result<Self,&'static str>{
        if args.len() < 3{ return Err("no enough arguments")}
        let ignore_case=env::var("IGNORE_CASE").is_ok();
        Ok(Self { 
            query:args[1].clone(), 
            file_path: args[2].clone(),
            ignore_case, 
        })
    }
}
pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents=fs::read_to_string(config.file_path)?;
    let results=if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    }else {
        search(&config.query, &contents)
    };
    for line in results{
        println!("{line}");
    }
    // for line in search(&config.query, &contents) {
    //     println!("{}",line);
    // }
    // .expect("Should have been able to read the file");
    // println!("With test:\n{contents}");
    Ok(())
}
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results=Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let query=query.to_lowercase();
    let mut results=Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}
#[cfg(test)]
mod tests{
    use super::* ;
    #[test]
    fn case_sensitive(){
        let query="duct";
        let contents="\
Rust:
safe,fast,productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe,fast,productive."],search(query,contents));
    }
    #[test]
    fn case_insensitive(){
        let query="rUsT";
        let contents="\
Rust:
safe,fast,productive.
Pick thtee.
Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query,contents));
    }
}