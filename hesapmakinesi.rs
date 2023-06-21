use std::io;

#[derive(Debug)] 
enum Operation {
    Addition(f64, f64),
    Subtraction(f64, f64),
    Multiplication(f64, f64),
    Division(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Addition(num1, num2) => num1 + num2,
        Operation::Subtraction(num1, num2) => num1 - num2,
        Operation::Multiplication(num1, num2) => num1 * num2,
        Operation::Division(num1, num2) => num1 / num2,
    }
}

fn main() {
    println!("İlk sayıyı girin:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Okuma hatası");

    println!("İşlemi girin (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Okuma hatası");

    println!("İkinci sayıyı girin:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Okuma hatası");

    let num1: f64 = num1.trim().parse().expect("Geçersiz sayı");
    let num2: f64 = num2.trim().parse().expect("Geçersiz sayı");

    let operation = match operator.trim() {
        "+" => Operation::Addition(num1, num2),
        "-" => Operation::Subtraction(num1, num2),
        "*" => Operation::Multiplication(num1, num2),
        "/" => Operation::Division(num1, num2),
        _ => panic!("Geçersiz işlem"),
    };

    let result = calculate(operation);
    println!("Sonuç: {}", result);
}