use std::io;
use std::collections::HashMap;
use std::collections::VecDeque;

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn is_string_number(s: &String) -> bool {
    s.chars().all(char::is_numeric)
}

fn shunting_yard(s: &String) -> Result<VecDeque<String>, String> {
    let mut output_queue: VecDeque<String> = VecDeque::new();
    let mut operator_stack: Vec<char> = Vec::new();

    let operators: HashMap<char, i32> = vec![
        ('+', 0), 
        ('-', 0), 
        ('/', 1), 
        ('*', 1), 
    ].into_iter().collect();

    let mut number_str: String = String::new();
    for (idx, ch) in s.chars().enumerate() {
        if ch.is_ascii_punctuation() {
            if !operators.contains_key(&ch) {
                let mut err_msg = "Invalid operator given: ".to_string();
                err_msg.push(ch);
                return Err(err_msg) 
            }
            if !number_str.is_empty() {
                output_queue.push_back(number_str.clone());
                number_str.clear();
            }

            while operator_stack.len() != 0 {
                let top_operator = operator_stack.last().unwrap();
                if operators[&top_operator] >= operators[&ch] {
                    let popped_operator = operator_stack.pop().unwrap();
                    output_queue.push_back(popped_operator.to_string());
                } else {
                    break;
                }
            }
            operator_stack.push(ch);
        } else if ch.is_ascii_digit() {
            number_str.push(ch);
            if idx == s.len() - 1 {
                output_queue.push_back(number_str.clone());
                number_str.clear();
            }
        } else {
            return Err("Invalid input".to_string())
        }
    }
    while operator_stack.len() != 0 {
        let popped_operator = operator_stack.pop().unwrap();
        output_queue.push_back(popped_operator.to_string());
    }
    
    return Ok(output_queue)
}

fn calculate_polish(values: &mut VecDeque<String>) -> Result<f32, String> {
    let mut number_stack: Vec<f32> = Vec::new();
    while !values.is_empty() {
        let value = match values.pop_front() {
            Some(val) => val,
            _ => return Err("This shouldn't happen".to_string())
        };
        if is_string_number(&value) {
            number_stack.push(value.parse::<f32>().unwrap())
        }
        else {
            let second_val = match number_stack.pop() {
                Some(val) => val,
                _ => return Err("Invalid input".to_string())
            };
            let first_val = match number_stack.pop() {
                Some(val) => val,
                _ => return Err("Invalid input".to_string())
            };
            match value.as_str() {
                "+" => {
                    let final_val = first_val + second_val;
                    number_stack.push(final_val);
                }
                "-" => {
                    let final_val = first_val - second_val;
                    number_stack.push(final_val);
                }
                "*" => {
                    let final_val = first_val * second_val;
                    number_stack.push(final_val);
                }
                "/" => {
                    let final_val = first_val / second_val;
                    number_stack.push(final_val);
                }
                _ => {
                    return Err("Wrong operator!!".to_string())
                }
            }
        }
    }
    return Ok(number_stack.pop().unwrap())
}

fn main() {
    loop {
        println!("Enter an expression to calculate");

        let mut expression = String::new();
        io::stdin()
            .read_line(&mut expression)
            .expect("failed to read line");
        
        remove_whitespace(&mut expression);
    
        let mut values: VecDeque<String> = match shunting_yard(&expression) {
            Ok(val) => val,
            Err(error) => {
                println!("{}\n", error);
                continue
            }
        }; 
        match calculate_polish(&mut values) {
            Ok(val) => {
                println!("Calculated number is: {}\n", val);
            },
            Err(error) => {
                println!("{}\n", error);
            }
        };
    }

}
