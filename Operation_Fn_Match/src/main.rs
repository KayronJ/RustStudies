use std::io::stdin;

fn sum(numb1: i32, numb2: i32){
    let plus: i32 = numb1 + numb2;
    println!("\nYou choose Sum.");
    println!("The result of {} + {} is = {}", numb1, numb2, plus);
}

fn subtration(numb1: i32, numb2: i32){
    let sub: i32 = numb1 - numb2;
    println!("\nYou choose Subtration.");
    println!("The result of {} - {} is = {}", numb1, numb2, sub);
}

fn multply(numb1: i32, numb2: i32){
    let mult: i32 = numb1 * numb2;
    println!("\nYou choose Multply.");
    println!("The result of {} * {} is = {}", numb1, numb2, mult);
}

fn division(numb1: i32, numb2: i32){
    let div: i32 = numb1 / numb2;
    println!("\nYou choose Multply.");
    println!("The result of {} / {} is = {}", numb1, numb2, div);
}

fn main() {
    
    let numb1: i32 = 5;
    let numb2: i32 = 5;

    let mut number = String::new();
    
    print!("Using {} and {} numbers, ", numb1, numb2);
    println!("please choose a operation option.\n");
    println!("1 - Adition");
    println!("2 - Subtration");
    println!("3 - Multply");
    println!("4 - Division");
    println!("Other to Exit Program\n");
    stdin().read_line(&mut number).expect("Failed input");  

    let option: i32 = number.trim().parse().unwrap();

    match option{
        1 => sum(numb1, numb2),
        2 => subtration(numb1, numb2),
        3 => multply(numb1, numb2),
        4 => division(numb1, numb2),
        _ => (),
    }
}