enum OpType {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Modulus, // %
    Pow, // ^
}

enum Element {
    Operator(OpType),
    Value(f32),
    Variable(String),
}

use stack::List;
use calc::OpType::*;
use calc::Element::*;
use std::str;

fn get_op_prioroty(op_ch: &char) -> i32 {
    match *op_ch {
        '(' => 0,
        ')' => 1,
        '+' | '-' => 2,
        '*' | '/' | '%' => 3,
        '^' => 4,
        _ => -1,
    }
}

fn get_polen_notation(expression: &String) -> Result<String, String> {
    let mut result = String::new();
    let mut stack = String::new();

    let mut operand = false;
    let mut prev_space = false;

    let mut idx = -1;

    'process: for c in expression.chars() {
        idx += 1;

        if prev_space && c != ' ' {
            result.push(' ');
            prev_space = false;
        }

        match c {
            '0'...'9' | '.' => {
                result.push(c);
                operand = true;
            }
            '(' => {
                stack.push(c);
            }
            ')' => {
                loop {
                    if (stack.is_empty()) {
                        return Err("Can't find closing parenthesis!".to_string());
                    }

                    let stack_char: char;
                    match (stack.pop()) {
                        Some(x) => stack_char = x,
                        None => return Err("End of stack is reached!".to_string()),
                    }

                    if (stack_char != '(') {
                        result.push(stack_char);
                    } else {
                        break;
                    }
                }
            }
            ' ' => {
                prev_space = true;
            }
            _ => {

                if !operand && c == '-' {
                    operand = true;
                    result.push(c);

                    continue 'process;
                }

                let prior = get_op_prioroty(&c);
                if (prior != -1) {
                    operand = false;
                    loop {
                        if !stack.is_empty() {
                            // get last operator priority
                            let stack_operator = &(stack.as_bytes()[stack.len() - 1] as char);

                            if prior <= get_op_prioroty(stack_operator) {
                                stack.pop();
                                result.push(*stack_operator);
                            } else {
                                break;
                            }

                        } else {
                            // return Err("Stack is empty".to_string());
                            break;
                        }
                    }
                    stack.push(c);
                    prev_space = true;
                } else {
                    return Err(format!("Unexpected char: <{}> at pos {}", c, &idx));
                }
            }
        }
    }

    while !stack.is_empty() {
        match stack.pop() {
            Some(x) => result.push(x),
            None => return Err("unexpected".to_string()),
        }
    }

    return Ok(result);
}

// fn read_f32(option: Option<f32>) -> f32 {
//     match option {
//         Some(x) => x,
//         None => panic!("Ohh, we got end of stack!"),
//     }
// }


fn calc_expression(expr_stack: List<Element>) -> Option<f32> {
    let mut val_stack: List<f32> = List::new();

    for x in expr_stack.iter() {
        match x {
            &Operator(ref op) => {
                let val2: f32;// = read_f32(val_stack.pop());
                let val1: f32;// = read_f32(val_stack.pop());

                match val_stack.pop() {
                    Some(x) => val2 = x,
                    None => return None,
                }

                match val_stack.pop() {
                    Some(x) => val1 = x,
                    None => return None,
                }

                val_stack.push(match *op {
                    Add => val1 + val2,
                    Sub => val1 - val2,
                    Mul => val1 * val2,
                    Div => val1 / val2,
                    Modulus => val1 % val2,
                    Pow => val1.powf(val2),
                });
            }

            &Value(ref val) => {
                val_stack.push(*val);
            }

            &Variable(ref var) => unimplemented!(),
        }
    }

    return match (val_stack.pop()) {
        Some(x) => {
            match val_stack.pop() {
                // not all values were used, it means that expression looks like this  "10 /(2 4)"
                Some(y) => {
                    return None;
                }

                None => Some(x),
            }
        }
        None => None,
    };
}

fn get_f32_from_string(val: &String) -> f32 {
    match val.parse::<f32>() {
        Ok(x) => x,
        Err(_) => 0.0,
    }
}

fn get_element_by_string(token: &String) -> Element {
    match (token.as_bytes()[0] as char) {
        '+' => Operator(Add),
        '-' => {
            if token.len() > 1 {
                Value(get_f32_from_string(token))
            } else {
                Operator(Sub)
            }
        }
        '*' => Operator(Mul),
        '/' => Operator(Div),
        '%' => Operator(Modulus),
        '^' => Operator(Pow),
        _ => Value(get_f32_from_string(token)),
    }
}


// fn print_stack(stack: &List<Element>) {
//     for x in stack.iter() {
//         print!("{} ",
//                match *x {
//                    Operator(Add) => "+".to_string(),
//                    Operator(Sub) => "-".to_string(),
//                    Operator(Mul) => "*".to_string(),
//                    Operator(Div) => "/".to_string(),
//                    Operator(Modulus) => "%".to_string(),
//                    Operator(Pow) => "^".to_string(),
//                    Value(val) => val.to_string(),
//                    Variable(_) => "ITS VAR".to_string(),
//                });
//     }
//     print!("/n");
// }

fn string_to_list(str_expr: &String) -> List<Element> {
    let mut tmp: List<Element> = List::new();
    let mut token: String = String::new();

    let mut operand = false;
    let mut prev_operator = false;

    'tokenize: for ch in str_expr.chars() {
        match ch {
            '0'...'9' | '.' => {
                token.push(ch);
                operand = true;
                prev_operator = false;
            }
            '+' | '-' | '*' | '/' | '%' | '^' => {

                if !operand && ch == '-' && !prev_operator {
                    token.push(ch);
                    operand = true;

                    continue 'tokenize;
                }

                if !token.is_empty() {
                    tmp.push(get_element_by_string(&(token)));
                    token.clear();
                }

                token.push(ch);

                tmp.push(get_element_by_string(&(token)));
                token.clear();

                operand = false;
                prev_operator = true;
            }
            ' ' => {
                prev_operator = false;
                if !token.is_empty() {
                    operand = false;
                    tmp.push(get_element_by_string(&(token)));
                    token.clear();
                }
            }
            _ => println!("unexpected situation"),
        }
    }

    // print_stack(&tmp);

    let mut stack: List<Element> = List::new();

    while let Some(el) = tmp.pop() {
        stack.push(el);
    }

    return stack;
}

pub fn calc(infix_str: &String) -> Result<f32, String> {
    let stack2;
    match get_polen_notation(infix_str) {
        Ok(x) => {
            stack2 = string_to_list(&x);

            match calc_expression(stack2) {
                Some(x) => Ok(x),
                None => Err("Incorrect expression!".to_string()),
            }
        }
        Err(x) => Err(x),
    }
}

#[cfg(test)]
mod calc_test {
    use super::calc;

    #[test]
    fn calc_basics() {

        let tests_result_4 = vec!["2+2",
                                  "2 +2",
                                  "2 + 2",
                                  " 2 + 2 ",
                                  "(2+2)",
                                  "( 2+2)",
                                  " ( 2 +2)",
                                  "(2+ 2)",
                                  "(2+ 2 )",
                                  "(2+ 2 ) ",
                                  "(((2+2)))",
                                  "(((2)) + 2)",
                                  "4-0",
                                  "8/2",
                                  "8*0.5",
                                  "9.5 - 5.5",
                                  "9 % 5",
                                  "-4 + 8",
                                  "8 + -4",
                                  "-2 + 6"];

        for test in tests_result_4 {
            match calc(&test.to_string()) {
                Ok(x) => assert_eq!(x, 4.0),
                Err(x) => {
                    println!("FAILED TEST: {}, error: {} \n", test, x);
                    assert!(false);
                }
            }
        }

    }

    #[test]
    fn calc_long_expression() {
        let tests_result_2648 = vec!["55 * (3552 / 74) + 2^3",
                                     "55 * (3552 / (37 * 2)) + 2^3",
                                     "(54 + 1) * (3552 / (37 * 2)) + 2^3",
                                     "55 * (3552 / (147 % 74 + 1)) + 2^3",
                                     "55 * (3552 / 74) + 2^(2+1)",
                                     "55 * (3552 / 74) + 2^(4-1)",
                                     "55 * (3552 / 74) + 2^(0.5 * 6)",
                                     "55 * (3552 / 74) + 2^(6 * 0.5)",
                                     "2^3 + 55 / (1/(3552 * (1/74)))",
                                     "55 * (3552 / (74-(-74+74)) ) + 2^3",
                                     "55 * (3552 / 74) + 2^3 - (-69 - 9) + (-69 - 9)"];

        for test in tests_result_2648 {
            match calc(&test.to_string()) {
                Ok(x) => assert_eq!(x, 2648.0),
                Err(x) => {
                    println!("FAILED TEST: {}, error: {} \n", test, x);
                    assert!(false);
                }
            }
        }
    }
}
