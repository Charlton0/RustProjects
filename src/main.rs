use std::io;

struct Bill {  //defining the struct
    name: String,
    amount: f64,
}

//functions go here
fn add_bill(bills: &mut Vec<Bill>) {

    let mut name = String::new();

    println!("Enter bill name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    let name = name.trim().to_string();

    let mut amount_input = String::new();

    println!("Enter bill amount:");

    io::stdin()
        .read_line(&mut amount_input)
        .expect("Failed to read input");

    let amount: f64 = amount_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let bill = Bill {
        name,
        amount,
    };

    bills.push(bill);

    println!("Bill added successfully!");
}

fn view_bills(bills: &Vec<Bill>) {

    if bills.is_empty() {

        println!("No bills available.");
        return;
    }

    println!("\n===== ALL BILLS =====");

    for (index, bill) in bills.iter().enumerate() {

        println!("-------------------");
        println!("Bill Number: {}", index);
        println!("Bill Name: {}", bill.name);
        println!("Amount: {}", bill.amount);
    }
}

fn remove_bill(bills: &mut Vec<Bill>) {

    if bills.is_empty() {
        println!("No bills to remove.");
        return;
    }

    println!("\nEnter bill number to remove:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    if index < bills.len() {
        bills.remove(index);
        println!("Bill removed successfully!");
    } else {
        println!("Invalid bill number.");
    }
}


fn edit_bill(bills: &mut Vec<Bill>) {

    if bills.is_empty() {
        println!("No bills available to edit.");
        return;
    }

    println!("\nEnter bill number to edit:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    if let Some(bill) = bills.get_mut(index) {

        let mut new_name = String::new();

        println!("Enter new bill name:");

        io::stdin()
            .read_line(&mut new_name)
            .expect("Failed to read input");

        let new_name = new_name.trim().to_string();

        let mut new_amount_input = String::new();

        println!("Enter new bill amount:");

        io::stdin()
            .read_line(&mut new_amount_input)
            .expect("Failed to read input");

        let new_amount: f64 = match new_amount_input.trim().parse() {
            Ok(amount) => amount,
            Err(_) => {
                println!("Please enter a valid amount.");
                return;
            }
        };

        bill.name = new_name;
        bill.amount = new_amount;

        println!("Bill updated successfully!");

    } else {
        println!("Invalid bill number.");
    }
}

fn main() {

    let mut bills: Vec<Bill> = Vec::new();

    loop {

        println!("\n===== BILL MANAGER =====");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Edit Bill");
        println!("5. Exit");

        println!("Enter your choice:");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice = choice.trim();

        match choice {

           "1" => {
    add_bill(&mut bills);
}

           "2" => {
    view_bills(&bills);
}

           "3" => {
    remove_bill(&mut bills);
}

           "4" => {
    edit_bill(&mut bills);
}

            "5" => {
                println!("Exiting program...");
                break;
            }

            _ => {
                println!("Invalid choice");
            }
        }
    }
}