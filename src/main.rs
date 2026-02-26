use std::io;
fn multiply_matrix(a: &[i32], b: &[i32]) -> [i32; 8] {
    let mut result = [0; 8];
    for i in 0..a.len() {
        for j in 0..b.len() {
            result[i * b.len() + j] += a[i] * b[j];
        }
    }
    result
}

fn main() {
    println!("calculator");
    let mut a: [i32; 8] = [0; 8];
    for i in 0..a.len() {
        println!("enter a value {};", i + 1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed");
        a[i] = input.trim().parse().expect("not a num");
    }
    let b: [i32; 8] = [2, 4, 6, 8, 10, 12, 14, 16];
    let result = multiply_matrix(&a, &b);
    println!("\n user array");
    for x in a {
        print!("{x} ");
    }
    println!("\n new array");
    for x in result {
        print!("{x} ");
    }
}
