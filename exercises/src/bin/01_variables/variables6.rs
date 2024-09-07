// TODO: Change the line below to fix the compiler error.
const NUMBER: usize = 3;

fn main() {
    println!("Number: {NUMBER}");
    {
        const NR1: bool = true;
        println!("NR1 and Number = {},{}", NR1, NUMBER );
    }
    // NR1 is not in this scope
    // println!("NR1 and Number = {},{}", NR1, NUMBER );
}
