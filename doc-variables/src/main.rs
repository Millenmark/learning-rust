fn main() {
    // let x = 5; # this will not compile since x is immutable
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    let a = 5;

    let a = a + 1;

    println!("The value of a after increment and before the inner scope is: {a}");

    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}");
    }

    // let a = 56;

    println!("The value of a in the outer scope is: {a}");
}
