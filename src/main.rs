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

       for (index, bill) in bills.iter().enumerate() {
        println!("-------------------");
        println!("Bill Number: {}", index);
        println!("Bill Name: {}", bill.name);
        println!("Amount: {}", bill.amount);
    }

}

fn remove_bill(bills: &mut Vec<Bill>, index: usize) {

    if index < bills.len() {
        bills.remove(index);
        println!("Bill removed successfully!");
    } else {
        println!("Invalid bill index.");
    }
}


fn edit_bill(
    bills: &mut Vec<Bill>,
    index: usize,
    new_name: String,
    new_amount: f64,
) {

    if let Some(bill) = bills.get_mut(index) {

        bill.name = new_name;
        bill.amount = new_amount;

        println!("Bill updated successfully!");

    } else {
        println!("Bill not found.");
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
                println!("View Bills selected");
            }

            "3" => {
                println!("Remove Bill selected");
            }

            "4" => {
                println!("Edit Bill selected");
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