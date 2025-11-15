fn main() {
    let parentheses="}";
    if is_valid_parentheses(parentheses) {
        println!("Success!");
    }else {
        println!("Failure!");
    }
}

fn is_valid_parentheses(s:&str)->bool{
    let mut stack:Vec<char>=Vec::new();
    //&str没有迭代器
    for c in s.chars() {
        match c {
            '('|'['|'{'=>stack.push(c),
            ')'=>{if stack.pop().expect("栈为空但遇到')'") !='(' {return false;}},
            ']'=>{if stack.pop().expect("栈为空但遇到']'") !='[' {return false;}},
            '}'=>{if stack.pop().expect("栈为空但遇到'}'") !='{' {return false;}},
            _ =>{},
        }
    }
    stack.is_empty()
}