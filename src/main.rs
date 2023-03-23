
use std::io::*;
use rand::Rng;
fn main() {
    task1();
}

fn task1(){
    struct person{
        name: String,
        surname: String,
        height: i32,
    }
    let mut people: Vec<person> = Vec::new();
    let mut check = true;
    let mut sum = 0;
    while check {
        println!("Enter name");
        let mut name = String::new();
        stdin().read_line(&mut name).expect("Failed to read line");
        println!("Enter surname");
        let mut surname = String::new();
        stdin().read_line(&mut surname).expect("Failed to read line");
        println!("Enter height");
        let mut height = String::new();
        stdin().read_line(&mut height).expect("Failed to read line");
        let height: i32 = height.trim().parse().expect("Please type a number!");
        let person = person{
            name: name.trim().to_string(),
            surname: surname.trim().to_string(),
            height: height,
        };
        sum += height;
        people.push(person);
        println!("Do you want to continue? (y/n)");
        let mut answer = String::new();
        stdin().read_line(&mut answer).expect("Failed to read line");
        if answer.trim() == "No" || answer.trim() == "no" || answer.trim() == "n" || answer.trim() == "N" {
            check = false;
        }
    }
    let avarage = sum / people.len() as i32;
    println!("Avarage height is {}", avarage);
    let mut difference = 0;
    let mut highestDifference = &people[0];
    for person in &people {
        if ((avarage-person.height).abs()) > difference {
            difference = (avarage-person.height).abs();
            highestDifference = &person;
        }
    }
    println!("Highest difference is {} {} and it is {}", highestDifference.name, highestDifference.surname, difference);


}
