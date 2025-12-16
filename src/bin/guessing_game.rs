use std::{cmp::Ordering, io};
use rand::random_range;
fn main() {
    println!("Guess the number!!");


    //生成随机数
    let secret_number=random_range(1..=100);
    loop {
        println!("Please input your gues.");
        let mut guess=String::new();
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        
        //String 实例的 trim 方法会去除字符串开头和结尾的空白字符
        //字符串的 parse 方法 将字符串转换成其他类型。我们需要告诉 Rust 具体的数字类型，
        // let guess:i32=guess.trim().parse().expect("Please type a number!");
        let guess:i32 =match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>continue,
        } ;
        println!("You guessed:{guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                println!("number:{secret_number}");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
