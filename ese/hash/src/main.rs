use std::io;
use std::collections::HashMap;

fn main() {
    let mut employee_list = HashMap::new();

    println!("What operation would you like to do?");
    print!("1. Add Employee\n2. Remove Employee\n");

    let mut choice_s = String::new();

    io::stdin()
        .read_line(&mut choice_s)
        .expect("Failed to read line!");

    let choice = choice_s.trim();
    let i:u32 = match choice.parse::<u32>() {
        Ok(i) => i,
        Err(choice) => 99,
    };


    //reads from user input name and department of the employee via the public function get_employe
    //e()

    let mut v = get_employee();


    match i {
        //adds employee
        1 => employee_list.insert(&String::from(v[0]), &String::from(v[1])),
        //removes employee
        2 => employee_list.remove(&String::from(v[0])),
        };

    println!("{:?}", employee_list);


}

pub fn get_employee() -> Vec<String> {
    let mut v = vec![];

    // reads from the user tha name and the department of the employee

    let mut name = String::new();
    let mut dep = String::new();

    println!("What is the name of the employee you'd like to add?");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line!");

    println!("What department will he be working in?");
    io::stdin()
        .read_line(&mut dep)
        .expect("Failed to read line!");

    v.push(name);
    v.push(dep);

    return v

}