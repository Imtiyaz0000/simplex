use simplex::*;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let mut objective: Vec<f64> = Vec::new();
    let mut constraints: Vec<SimplexConstraint> = Vec::new();
    let var_num: usize;
    let program: Result<SimplexTable, String>;
    let constraint_num: usize;
    let max: bool;
    println!("Welcome to the simplex algorithm solver!!11! (by Imtiyaz🤑)");
    thread::sleep(Duration::from_secs(3));
    loop {
        println!("Please enter the number of variables: 🔥");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => {
                println!("Failed to take input: {}", error);
                thread::sleep(Duration::from_secs(3));
                continue;
            }
        }
        var_num = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!(
                    "Failed to parse input: {} (Make sure you entered a number💀)",
                    e
                );
                continue;
            }
        };
        break;
    }

    {
        println!("Enter the coefficients of the objective function: 🔥🔥🔥");
        let mut i = 1;
        loop {
            println!("Enter the coefficient for x{}: ", i);
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(e) => {
                    println!("Failed to take input: {} ☠️", e);
                    continue;
                }
            }
            objective.push(match input.trim().parse() {
                Ok(num) => num,
                Err(e) => {
                    println!(
                        "Failed to parse input: {} (Make sure you entered a number) 💀☠️",
                        e
                    );
                    continue;
                }
            });
            i += 1;
            if i == var_num + 1 {
                break;
            };
        }
    }

    println!("Lets now set up the constraints!🤩: ");

    loop {
        println!("Enter the number of constraints👀: ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to take input: {} ☠️", e);
                continue;
            }
        }

        constraint_num = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!(
                    "Failed to parse input: {} (Make sure you entered a number)☠️☠️",
                    e
                );
                continue;
            }
        };
        break;
    }

    {
        println!("Enter the coefficients of the constraints👀: ");
        let mut i: usize = 1;
        loop {
            let mut temp_constraint: Vec<f64> = Vec::new();
            println!("Lets set up constrain no. {} 😎", i);
            let mut j: usize = 1;
            loop {
                println!("Enter the coefficient for x{}: ", j);
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Failed to take input: {} ☠️", e);
                        continue;
                    }
                }
                temp_constraint.push(match input.trim().parse() {
                    Ok(num) => num,
                    Err(e) => {
                        println!(
                            "Failed to parse input: {} (Make sure you entered a number)💀💀",
                            e
                        );
                        continue;
                    }
                });
                j += 1;
                if j == var_num + 1 {
                    break;
                }
            }

            loop {
                println!("Is this constraint a less than or greater than constraint? (l/g)😝");
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Failed to take input: {}💀☠️", e);
                        continue;
                    }
                }

                if input.trim() != "l" && input.trim() != "g" {
                    println!("Please enter either 'l' or 'g' ☠️☠️☠️☠️☠️");
                    continue;
                }

                loop {
                    println!("Enter the right hand side of the constraint: 😭");
                    let mut right = String::new();
                    match io::stdin().read_line(&mut right) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("Failed to take input: {} ☠️☠️", e);
                            continue;
                        }
                    }
                    let right: f64 = match right.trim().parse() {
                        Ok(num) => num,
                        Err(e) => {
                            println!(
                                "Failed to parse input: {} (Make sure you entered a number) 💀",
                                e
                            );
                            continue;
                        }
                    };

                    if input.trim() == "l" {
                        constraints.push(SimplexConstraint::LessThan(temp_constraint, right));
                    } else if input.trim() == "g" {
                        constraints.push(SimplexConstraint::GreaterThan(temp_constraint, right));
                    }
                    break;
                }
                break;
            }
            i += 1;
            if i == constraint_num + 1 {
                break;
            }
        }
    }
    loop {
        println!("Is this a maximise or minimise problem? (min/max): 🤑🤑🤑");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                println!("Error while reading input: {} 💀💀💀", e);
                continue;
            }
        }
        if input.trim() == "min" {
            max = false;
            program = Simplex::minimize(&objective).with(constraints);
            break;
        } else if input.trim() == "max" {
            max = true;
            for i in objective.iter_mut() {
                *i *= -1.0;
            }
            program = Simplex::minimize(&objective).with(constraints);
            break;
        } else {
            println!("Please enter either 'min' or 'max' 😭");
            continue;
        }
    }

    let mut simplex = program.unwrap();

    let mult: f64 = if max {
        -1.0
    } else {
        1.0
    };
    match simplex.solve() {
        SimplexOutput::UniqueOptimum(optimum) => {
            println!("The unique optimum is: {} 😎", optimum * mult);
        }
        SimplexOutput::MultipleOptimum(optimum) => {
            println!("The optimum is: {} 😎", optimum * mult);
        }
        SimplexOutput::InfiniteSolution => {
            println!("The problem has infinite solutions 😳");
        }
        SimplexOutput::NoSolution => {
            println!("The problem has no solution 🥺");
        }
    }
    for i in 1..=var_num {
        println!("optimal value of x{i} = {} 👀", simplex.get_var(i).unwrap());
    }
    println!("press enter to exit 🥺🥺🥺");

    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}
