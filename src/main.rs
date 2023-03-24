
use std::{io::*, vec};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //task1();
    //task2();
    //task3();
    //task4();
    task5();
}


struct Fruit{
    name: String,
    color: String,
    readyForSale: bool,
}

struct MultiFruit{
    name: String,
    color: Vec<String>,
    readyForSale: bool,
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
fn task2(){
    let mut numberOfPeopleTry = String::new();
    stdin().read_line(&mut numberOfPeopleTry).expect("Failed to read line");
    let numberOfPeople: i32 = numberOfPeopleTry.trim().parse().expect("Please type a number!");
    struct Person{
        name: String,
        surname: String,
        height: i32,
        hairColor: String,
    }
    let mut people: Vec<Person> = Vec::new();
    for i in 0..numberOfPeople {
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
        println!("Enter hair color");
        let mut hairColor = String::new();
        stdin().read_line(&mut hairColor).expect("Failed to read line");
        let person = Person{
            name: name.trim().to_string(),
            surname: surname.trim().to_string(),
            height: height,
            hairColor: hairColor.trim().to_string(),
        };
        people.push(person);
    }
    people.sort_by(|a,b| a.height.cmp(&b.height));
    let middleIndex: usize = numberOfPeople as usize / 2;
    //let qualifiedPeople = &people[middleIndex..numberOfPeople as usize];
    let mut usedColors: Vec<String> = Vec::new();
    let mut maxPeople = 0;
    let mut maxColor = String::new();
    for person in middleIndex..numberOfPeople as usize {
        if usedColors.contains(&people[person].hairColor) {
            continue
        }
        let mut count = 0;
        usedColors.push(people[person].hairColor.clone());
        /*for j in middleIndex..numberOfPeople as usize {
            if people[person].hairColor == people[j].hairColor {
                count += 1;
            }
        }*/
        let tempClone = &people[middleIndex..numberOfPeople as usize];
        let sameColor = tempClone.iter().filter(|&x| x.hairColor == people[person].hairColor).count();
        if sameColor > maxPeople {
            maxPeople = sameColor;
            maxColor = people[person].hairColor.clone();
            println!("{} {}", maxPeople, maxColor)
        }
    }
    println!("Most popular color is {} and it is used by {} people", maxColor, maxPeople);
        
    }  
fn task3(){
    let mut fruits: Vec<Fruit> = Vec::new();
    let mut check = true;
    while check {
        println!("Enter name");
        let mut name = String::new();
        stdin().read_line(&mut name).expect("Failed to read line");
        println!("Enter color");
        let mut color = String::new();
        stdin().read_line(&mut color).expect("Failed to read line");
        println!("Enter ready for sale (true/false)");
        let mut readyForSale = String::new();
        stdin().read_line(&mut readyForSale).expect("Failed to read line");
        let readyForSale: bool = readyForSale.trim().parse().expect("Please type a number!");
        let fruit = Fruit{
            name: name.trim().to_string(),
            color: color.trim().to_string(),
            readyForSale: readyForSale,
        };
        fruits.push(fruit);
        println!("Do you want to continue? (y/n)");
        let mut answer = String::new();
        stdin().read_line(&mut answer).expect("Failed to read line");
        if answer.trim() == "No" || answer.trim() == "no" || answer.trim() == "n" || answer.trim() == "N" {
            check = false;
        }
    }
    for fruit in &fruits {
        if fruit.color == fruit.name {
            println!("{} {} has the same name and color", fruit.name, fruit.color);
        }
    }
}
fn task4(){
    let mut multiColoredFruits: Vec<MultiFruit>= Vec::new();
    let mut check = true;
    while check {
        println!("Enter name");
        let mut name = String::new();
        stdin().read_line(&mut name).expect("Failed to read line");
        let mut amountOfColorsTry = String::new();
        stdin().read_line(&mut amountOfColorsTry).expect("Failed to read line");
        let amountOfColors: i32 = amountOfColorsTry.trim().parse().expect("Please type a number!");
        let mut colors: Vec<String> = Vec::new();
        for i in 0..amountOfColors {
            println!("Enter color");
            let mut color = String::new();
            stdin().read_line(&mut color).expect("Failed to read line");
            colors.push(color.trim().to_string());
        }
        let mut color = String::new();
        stdin().read_line(&mut color).expect("Failed to read line");
        println!("Enter ready for sale (true/false)");
        let mut readyForSale = String::new();
        stdin().read_line(&mut readyForSale).expect("Failed to read line");
        let readyForSale: bool = readyForSale.trim().parse().expect("Please type a number!");
        let fruit = MultiFruit{
            name: name.trim().to_string(),
            color: colors,
            readyForSale: readyForSale,
        };
        multiColoredFruits.push(fruit);
        println!("Do you want to continue? (y/n)");
        let mut answer = String::new();
        stdin().read_line(&mut answer).expect("Failed to read line");
        if answer.trim() == "No" || answer.trim() == "no" || answer.trim() == "n" || answer.trim() == "N" {
            check = false;
        }
    }
    let forbidden =  String::from("yellow");
    for  i in 0..multiColoredFruits.len() {
        println!("Old fruit: {} {:?}", multiColoredFruits[i].name, multiColoredFruits[i].color);
        for j in 0..multiColoredFruits[i].color.len() {
            if multiColoredFruits[i].color[j] == forbidden {
                let index = multiColoredFruits[i].color.iter().position(|x| x == &multiColoredFruits[i].color[j]).unwrap();
                multiColoredFruits[i].color[index] = "red".to_string();
            }
        }
        println!("New fruit: {} {:?}", multiColoredFruits[i].name, multiColoredFruits[i].color);
    }

}
fn task5(){
    let mut fruits: Vec<MultiFruit> = Vec::new();
    let mut check = true;
    while check {
        println!("Enter name");
        let mut name = String::new();
        stdin().read_line(&mut name).expect("Failed to read line");
        let mut amountOfColorsTry = String::new();
        stdin().read_line(&mut amountOfColorsTry).expect("Failed to read line");
        let amountOfColors: i32 = amountOfColorsTry.trim().parse().expect("Please type a number!");
        let mut colors: Vec<String> = Vec::new();
        for i in 0..amountOfColors {
            println!("Enter color");
            let mut color = String::new();
            stdin().read_line(&mut color).expect("Failed to read line");
            colors.push(color.trim().to_string());
        }
        let mut color = String::new();
        stdin().read_line(&mut color).expect("Failed to read line");
        println!("Enter ready for sale (true/false)");
        let mut readyForSale = String::new();
        stdin().read_line(&mut readyForSale).expect("Failed to read line");
        let readyForSale: bool = readyForSale.trim().parse().expect("Please type a number!");
        let fruit = MultiFruit{
            name: name.trim().to_string(),
            color: colors,
            readyForSale: readyForSale,
        };
        fruits.push(fruit);
        println!("Do you want to continue? (y/n)");
        let mut answer = String::new();
        stdin().read_line(&mut answer).expect("Failed to read line");
        if answer.trim() == "No" || answer.trim() == "no" || answer.trim() == "n" || answer.trim() == "N" {
            check = false;
        }
    }
    let yellowFruits: Vec<&MultiFruit> = fruits.iter().filter(|&x| x.color.contains(&String::from("yellow"))).collect();
    for fruit in &yellowFruits {
        println!("{}", fruit.name);
    }

}