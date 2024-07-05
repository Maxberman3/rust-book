use std::{collections::HashMap, io::stdin, process};

use regex::Regex;

fn main() {
    let mut department_list: HashMap<String, Vec<String>> = HashMap::new();
   
    loop {
        let mut operation = String::new();
        println!("Would you like to retrieve a department list, or add an employee to a department? Answer \"Add\" or \"List\" or \"Exit\"");
        stdin()
            .read_line(&mut operation)
            .expect("failed to read line");

        let operation_lowercase = operation.trim().to_lowercase();
        assert!(
            operation_lowercase == "add"
                || operation_lowercase == "list"
                || operation_lowercase == "exit",
            "You must answer with either \"Add\", \"List\" or \"Exit\""
        );
        
        if operation_lowercase == "exit"{
            process::exit(0);
        } else if operation_lowercase == "add" {
            add_employee_operation(&mut department_list);
        } else if operation_lowercase == "list" {
            list_employee_operation(&department_list)
        }
    }
}

fn list_employee_operation(department_list: &HashMap<String, Vec<String>>){
    println!("Please state which department you would like to list employees for:");
    let mut department = String::new();
    stdin()
        .read_line(&mut department)
        .expect("Failed to read line");

    let department_lowercase = department.trim().to_lowercase();
    let employees_entry = department_list.get(&department_lowercase);

    match employees_entry {
        Some(employees) => println!("{:?}", employees),
        None => println!("The department you requested doesn't exist!")
    }
}

fn add_employee_operation(department_list: &mut HashMap<String, Vec<String>>) {
    let mut employee_add_statement = String::new();
    let regex_test = Regex::new(r"^add (\w+) to (\w+)$").unwrap();
    println!("Please state which employee to add to which department in the following format \"Add <name> to <department>\"");

    stdin()
        .read_line(&mut employee_add_statement)
        .expect("failed to read line");

    let employee_add_statement_lowercase = employee_add_statement.trim().to_lowercase();
    assert!(
        regex_test.is_match(&employee_add_statement_lowercase),
        "Please make sure that your statement matches the defined format"
    );

    let words: Vec<&str> = employee_add_statement_lowercase
        .split_whitespace()
        .collect();
    let name = words[1];
    let department = words[3];

    let department_employees = department_list
        .entry(department.to_lowercase().to_string())
        .or_insert(Vec::new());
    department_employees.push(name.to_string());

    // println!("Departments:");
    // for (key, value) in department_list {
    //     println!("{}: {:?}", key, value);
    // }
}
