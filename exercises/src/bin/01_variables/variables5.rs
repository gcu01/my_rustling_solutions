fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    println!("Number plus two is: {}", number + 2);
    let n1:i32 = number;
    println!("n1 is: {}", n1);
    consume(number);
    println!("Number plus two is: {}", number + 2);
}

fn consume(n: i32) {
    println!("I am consuming the {}", n);
}