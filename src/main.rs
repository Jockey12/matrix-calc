use std::io;
fn multiply_matrix(a: &[i32], b: &[i32]) -> [i32; 8] {
    let mut result = [0; 8];
    let n = a.len().min(b.len()).min(result.len());
    for i in 0..n {
        result[i] = a[i] * b[i];
    }
    result
}

fn main() {
    println!("calculator");
    let mut a: [i32; 8] = [0; 8];
    let mut b: [i32; 8] = [0; 8];
    for i in 0..a.len() {
        println!("enter a value {};", i + 1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed");
        a[i] = input.trim().parse().expect("not a num");
    }
    for i in 0..b.len() {
        println!("enter a value {};", i + 1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed");
        b[i] = input.trim().parse().expect("not a num");
    }
    let result = multiply_matrix(&a, &b);
    println!("\n  array A");
    for x in a {
        print!("{x} ");
    }
    println!("\n array B");
    for x in b {
        print!("{x} ");
    }
    println!("\n new array");
    for x in result {
        print!("{x} ");
    }
}
