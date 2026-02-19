// use the functions from the functions module in the utils module
mod utils;

fn main() {
    // use the functions from the utils module
    let sum = utils::functions::add(5, 3);
    let difference = utils::functions::subtract(5, 3);
    let product = utils::functions::multiply(5, 3);
    let quotient = utils::functions::divide(5, 3); 
    println!("Sum: {sum}, Difference: {difference}, Product: {product}, Quotient: {quotient:?}");
}
