
use std::io;
use std::collections::HashMap;


fn main() -> ! {
    

    let mut departments_with_employees: HashMap<String, Vec<String>> = HashMap::new();
    let mut companies: HashMap<String, Option<HashMap<String, Vec<String>>>> = HashMap::new();


    loop {

        menu();

        let mut menu_option = String::new();
        io::stdin().read_line(&mut menu_option).expect("Failed to read line");

        match menu_option.trim().parse() {

            Ok(1) => { add_company(&mut companies);
            }
            Ok(2) => { 

                let company = company_choice(companies.keys().map(|k| k.as_str()).collect());
                let companyyyy;
                if company == 0{
                    continue;
                } else {
                    companyyyy = companies.keys().nth(company + 1).unwrap();
                }


                'outer: loop {

                    let (employee, department) = employee_and_department_input(&companyyyy);
                    add_employee_to_department(employee, department, &mut departments_with_employees);

                    
                    loop {

                        println!("Add more? y/n");
                        let mut y_n = String::new();
                        io::stdin().read_line(&mut y_n).expect("Failed to read line");

                        let answer_char = match y_n.trim().chars().next() {
                            Some(char) => char,
                            None => continue
                        };

                        if answer_char == 'y' {
                            continue 'outer;
                        } else if answer_char == 'n' {
                            break 'outer
                        } else {
                            continue;
                        }


                    }
                    
                }
               


            }
            Ok(3) => { 
                
                for companies in &companies {
                    println!("{:?}", companies);
                }

            }
            Ok(4) => {
                let keys: Vec<&str> = companies.keys().map(|k| k.as_str()).collect();
                 print_companies(&keys); 
                }
            _ => { 
                println!("Invalid input");
                continue;
            }
        };

        
        



    }


}


fn menu() {
    
    println!("                                  Add Company -> 1");
    println!("                                 Add employee -> 2");
    println!("                Print employees of department -> 3");
    println!("Print employees of all departments of company -> 4");
}

fn add_company(companies: &mut HashMap<String, Option<HashMap<String, Vec<String>>>>) {
    
    let mut company_name = String::new();
    io::stdin().read_line(&mut company_name).expect("failed to read line");

    companies.insert(company_name.trim().to_string(), None);

    
    
}

fn employee_and_department_input(company: &str) -> (String, String) {

    println!("\n");
    println!("Current company: {}", company);
    
    let mut employee = String::new();
    println!("Who do you wish to add: ");
    io::stdin().read_line(&mut employee).expect("Failed to read line");  

    println!();

    let mut department = String::new();
    print!("In which department do you want to add {}", employee);
    io::stdin().read_line(&mut department).expect("Failed to read line");

    (employee, department)
}



fn print_companies(company_map: &Vec<&str>) {

    for (index, companies) in company_map.iter().enumerate() {
        println!("{}) {}", index + 1, companies)
    }

}


fn add_employee_to_department(key:String, value:String, map: &mut HashMap<String, Vec<String>>) {
    

    if let Some(vec) = map.get_mut(&key) {

        vec.push(value);
 
    } else {
        map.insert(key, vec![value]);
    }

    

}


fn company_choice(companies: Vec<&str>) -> usize {


    
    print_companies(&companies);
    println!("Choose company to add employees: ");

    let mut chosen_company = String::new();
    io::stdin().read_line(&mut chosen_company).expect("Choose an index please");

    match chosen_company.trim().parse::<usize>() {

        Ok(index_key) => {
            if index_key == 0 {

                return 0;
                
            } else if index_key < companies.len(){
                println!("{}", index_key);
                return index_key + 1
            } else {
                println!("there is no company at index: {}", index_key);
            }
        }
        Err(_) => {
            println!("Choose a valid index.")
        }


    }

    company_choice(companies)

}
