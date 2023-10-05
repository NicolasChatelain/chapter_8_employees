use std::io;

fn main() {
    

    let mut companies_vector: Vec<String> = Vec::new();

    loop {

        menu();

        let mut menu_option = String::new();

        io::stdin().read_line(&mut menu_option).expect("Failed to read line");

        let user_choice = match menu_option.trim().parse() {

            Ok(1) => { companies_vector = add_company(&companies_vector);},
            Ok(2) => { option_2(); },
            Ok(3) => { option_3(); },
            Ok(4) => { option_4(&companies_vector); },
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

fn add_company(companies_vector: &Vec<String>) -> &Vec<String>{
    
    let mut company_name = String::new();
    io::stdin().read_line(&mut company_name).expect("failed to read line");

    companies_vector.push(company_name);
    &companies_vector
}

fn option_2() {
    println!("hello2");
}

fn option_3() {
    println!("hello3");
}

fn option_4(company_vector: &Vec<String>) {
    for companies in company_vector {
        println!("{}", companies)
    }
}

