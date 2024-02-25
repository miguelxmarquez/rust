/*  
*  Variables and Mutability
*/

fn main() {


    /* Floating-Point Type */
    let first = 2.0; // f64
    let second: f32 = 3.0; // f32
    println!("The value of the floating type is {first} and {second}");


    /* Numeric Operations */
    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let truncated = -5 / 3; // results in -1 
    let remainder = 43 % 5; // remainder
    println!("The value of all elements are: Sum: {sum}, Remainder: {remainder}, Quotient: {quotient}, Truncated: {truncated}, Difference: {difference}, Product: {product}");


    /* Boolean Type */
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The value of the variable t is: {t}");
    println!("The value of the variable f is: {f}");


    /* Char Type */
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("The value of character types is c: {c}");
    println!("The value of character types is z: {z}");
    println!("The value of character types is heart eyed cat: {heart_eyed_cat}");


    /* Mutable Variable */
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /* Inmutable Variable */
    let y = 5;
    let y = y + 1; // First assignment

    {// Redeclare Variable on Another Scope
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of original variable y is: {y}");


}