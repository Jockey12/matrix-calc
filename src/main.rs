fn multiply_matrix(_a: &[i32]) {
    let a: [i32; 24] = [0; 24];

    for x in a {
        print!("{x} ");
    }
}

fn main() {
    println!("calculator");
    let a: [i32; 8] = [0; 8];
    multiply_matrix(&a);
}
