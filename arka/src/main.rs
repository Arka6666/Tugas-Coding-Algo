use std::io;

#[derive(Debug)]
struct GymMember {
    member_id: String,
    name: String,
    membership_type: String,
}

fn add_member(member_list: &mut Vec<GymMember>) {
    println!("Add Gym Member");

    println!("Enter Member ID:");
    let mut member_id = String::new();
    io::stdin().read_line(&mut member_id).expect("Failed to read member ID");

    println!("Enter Member Name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read member name");

    println!("Enter Membership Type:");
    let mut membership_type = String::new();
    io::stdin()
        .read_line(&mut membership_type)
        .expect("Failed to read membership type");

    let new_member = GymMember {
        member_id: member_id.trim().to_string(),
        name: name.trim().to_string(),
        membership_type: membership_type.trim().to_string(),
    };

    member_list.push(new_member);
    println!("Gym member successfully added!");
}

fn display_members(member_list: &Vec<GymMember>) {
    println!("Gym Member Data");

    for member in member_list {
        println!("{:?}", member);
    }
}

fn edit_member(member_list: &mut Vec<GymMember>) {
    println!("Edit Gym Member");

    println!("Enter Member ID to edit:");
    let mut member_id = String::new();
    io::stdin().read_line(&mut member_id).expect("Failed to read member ID");

    if let Some(member) = member_list.iter_mut().find(|m| m.member_id.trim() == member_id.trim()) {
        println!("Enter New Member Name:");
        let mut new_name = String::new();
        io::stdin().read_line(&mut new_name).expect("Failed to read new member name");
        member.name = new_name.trim().to_string();

        println!("Enter New Membership Type:");
        let mut new_membership_type = String::new();
        io::stdin()
            .read_line(&mut new_membership_type)
            .expect("Failed to read new membership type");
        member.membership_type = new_membership_type.trim().to_string();

        println!("Gym member successfully edited!");
    } else {
        println!("Member with ID {} not found.", member_id.trim());
    }
}

fn delete_member(member_list: &mut Vec<GymMember>) {
    println!("Delete Gym Member");

    println!("Enter Member ID to delete:");
    let mut member_id = String::new();
    io::stdin().read_line(&mut member_id).expect("Failed to read member ID");

    if let Some(index) = member_list.iter().position(|m| m.member_id.trim() == member_id.trim()) {
        member_list.remove(index);
        println!("Gym member successfully deleted!");
    } else {
        println!("Member with ID {} not found.", member_id.trim());
    }
}

fn main() {
    let mut gym_members: Vec<GymMember> = Vec::new();

    loop {
        println!("Menu:");
        println!("1. Add Gym Member");
        println!("2. Display Gym Members");
        println!("3. Edit Gym Member");
        println!("4. Delete Gym Member");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Choice not available");

        match choice.trim().parse() {
            Ok(1) => add_member(&mut gym_members),
            Ok(2) => display_members(&gym_members),
            Ok(3) => edit_member(&mut gym_members),
            Ok(4) => delete_member(&mut gym_members),
            Ok(5) => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
