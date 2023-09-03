fn main() {
    //Defining inputs
    let mut input1 = String::new();
    let mut input2 = String::new();

    //Taking input1 from user
    println!("Please type first number");
    std::io::stdin().read_line(&mut input1).unwrap();
    //Validation of input1
    match input1.trim_end().parse::<f64>() {
        Ok(number) => {}
        Err(error_message) => println!("Input is in the wrong format!"),
    }
    
    //Taking input2 from user
    println!("Please type second number");
    std::io::stdin().read_line(&mut input2).unwrap();

    //Validation of input1
    match input2.trim_end().parse::<f64>() {
        Ok(number) => {}
        Err(error_message) => println!("Input is in the wrong format!"),
    }
    //converting input data
    let request_1 = input1.trim_end().parse::<f64>().unwrap();
    let request_2 = input2.trim_end().parse::<f64>().unwrap();

    //implementing each functions
    let operation1 = Operation::Add {num1: request_1,num2: request_2 };
    let operation2 = Operation::Divide {num1: request_1, num2: request_2 };
    let operation3 = Operation::Subtract {num1: request_1, num2:request_2 };
    let operation4 = Operation::Multiply {num1: request_1, num2:request_2 };


    let operation1_result = Calculate(operation1);
    let operation2_result = Calculate(operation2);
    let operation3_result = Calculate(operation3);
    let operation4_result = Calculate(operation4);

    //Printing the outputs
    match operation1_result {
        Ok(result) => println!("Adding result is {}", result),
        Err(error_message) => println!("Error: {}", error_message),
    }
    match operation2_result {
        Ok(result) => println!("Divide result is {}", result),
        Err(error_message) => println!("Error: {}", error_message),
    }
    match operation3_result {
        Ok(result) => println!("Subtract result is {}", result),
        Err(error_message) => println!("Error: {}", error_message),
    }
    match operation4_result {
        Ok(result) => println!("Multiply result is {}", result),
        Err(error_message) => println!("Error: {}", error_message),
    }
}

enum Operation {
    Add { num1: f64, num2: f64 },
    Subtract { num1: f64, num2: f64 },
    Multiply { num1: f64, num2: f64 },
    Divide { num1: f64, num2: f64 },
}
//Defining math operations
fn Calculate(input: Operation) -> Result<f64, String> {
    match input {
        Operation::Add { num1, num2 } => {
            return Ok(num1 + num2);
        }
        Operation::Divide { num1, num2 } => {
            if (num2 == 0.0) {
                let mut result_message = String::from("Cannot be divide by 0!  Wrong input: ");
                result_message.push_str(&num2.to_string());

                return Result::Err(result_message);
            } else {
                return Ok(num1 / num2);
            }
        }
        Operation::Multiply { num1, num2 } => {
            return Ok(num1 * num2);
        }
        Operation::Subtract { num1, num2 } => {
            return Ok(num1 - num2);
        }
    }
}
