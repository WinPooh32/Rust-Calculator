pub mod stack;

enum OpType{
    Add,             // +
    Sub,             // -
    Mul,             // *
    Div,             // /
    Modulus,         // %
    Pow,             // ^
}

enum Element{
    Operator(OpType),
    Value(f32),
    Variable(String)
}

use stack::List;
use OpType::*;
use Element::*;
use std::str;
use std::env;

fn get_op_prioroty(op_ch: &char) -> i32{
    match *op_ch{
        '('              => 0,
        ')'              => 1,
        '+' | '-'        => 2,
        '*' | '/' | '%'  => 3,
        '^'              => 4,
        _                => -1
    }
}

fn read_char(option: Option<char>) -> char{
    match option{
        Some(x) => x,
        None => panic!("Ohh, we got end of stack!")
    }
}

fn get_polen_notation(expression: &String) -> Result<String, String>{
    let mut result = String::new();
    let mut stack = String::new();
    let mut prev_space = false;

    for c in expression.chars(){

        if prev_space && c != ' ' {
            result.push(' ');
            prev_space = false;
        }

        match c{
            '0'...'9' | '.' =>{
                result.push(c);
            },
            '(' => stack.push(c),
            ')' =>{
                loop{
                    let stack_char = read_char(stack.pop());

                    if(stack.is_empty()){
                        return Err("Can't find closing parenthesis or unexpected delimiter.".to_string());
                    }

                    if(stack_char != '('){
                        result.push(stack_char);
                    }else{
                        break;
                    }
                }
            },
            ' ' => prev_space = true,
            _ => {
                let prior = get_op_prioroty(&c);
                if(prior != -1){
                    loop{
                        if !stack.is_empty() {
                            //get last operator priority
                            let stack_operator = &(stack.as_bytes()[stack.len() - 1] as char);
                            if prior <= get_op_prioroty(stack_operator) {
                                stack.pop();
                                result.push(*stack_operator);
                            }else{break;}
                        }else{ //return Err("Stack is empty".to_string());
                            break;
                        }
                    }
                    stack.push(c);
                    prev_space = true;
                }else{
                    return Err(format!("Unexpected char: <{}>, expected operator", c));
                }
            }
        }
    }

    while !stack.is_empty(){
        result.push(read_char(stack.pop()));
    }

    return Ok(result);
}

fn read_f32(option: Option<f32>) -> f32{
    match option{
        Some(x) => x,
        None => panic!("Ohh, we got end of stack!")
    }
}

fn calc_expression(expr_stack: List<Element>) -> f32{
    let mut val_stack: List<f32> = List::new();

    for x in expr_stack.iter(){
        match x{
            &Operator(ref op) => {
                let val2 = read_f32(val_stack.pop());
                let val1 = read_f32(val_stack.pop());

                val_stack.push( match *op {
                    Add => val1 + val2,
                    Sub => val1 - val2,
                    Mul => val1 * val2,
                    Div => val1 / val2,
                    Modulus => val1 % val2,
                    Pow => val1.powf(val2)
                });
            },

            &Value(ref val) => {
                val_stack.push(*val);
            },

            &Variable(ref var) => {
                unimplemented!()
            }
        }
    }

    return read_f32(val_stack.pop());
}

fn get_element_by_string(token: &String) -> Element{
    match (token.as_bytes()[0] as char){
        '+' => Operator(Add),
        '-' => Operator(Sub),
        '*' => Operator(Mul),
        '/' => Operator(Div),
        '%' => Operator(Modulus),
        _ => {
            println!("Token before parse: {}", token);
            match token.parse::<f32>(){
                Ok(x)  => {println!("{}", x); Value(x)},
                Err(x) => {println!("{}", x); Value(0.0)}
            }
        }
    }
}

fn string_to_list(str_expr: &String) -> List<Element>{
    let mut tmp: List<Element> = List::new();
    let mut token: String = String::new();

    for ch in str_expr.chars(){
        match ch{
            '0'...'9' | '.' => {
                token.push(ch);
            },
            '+'|'-'|'*'|'/'|'%' => {
                if !token.is_empty(){
                    tmp.push(get_element_by_string( &(token) ));
                    token.clear();
                }

                token.push(ch);
                tmp.push(get_element_by_string( &(token) ));
                token.clear();
            },
            ' ' => if !token.is_empty(){
                tmp.push(get_element_by_string( &(token) ));
                token.clear();
            },
            _ => {}
        }
    }

    let mut stack: List<Element> = List::new();

    while let Some(el) = tmp.pop(){
        stack.push(el);
    }

    return stack;
}

fn main() {
    let mut args_str = String::new();
    let mut is_silent = false;

    for argument in env::args().skip(1) {
        match argument.as_ref(){
            "-s" | "--silent" => is_silent = true,
            _ => args_str = args_str + " " + &argument
        }
    }

    let stack2;
    match get_polen_notation(&args_str) {
        Ok(x) => {
            stack2 = string_to_list(&x);
            if(is_silent){
                print!("{}", calc_expression(stack2));
            }else{
                println!("Result = {}", calc_expression(stack2));
            }
        },
        Err(x) => println!("{}", x)
    }
}
