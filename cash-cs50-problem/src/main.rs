//This is a simple solution with at least 4 functions is to calcilate pennies,dimes,quaters and nickels
//This excludes the main function
fn calc_pennies(cents: i32) -> i32 {
    cents / 1
}
fn calc_dimes(cents: i32) -> i32 {
    cents / 10
   
}
fn calc_quaters(cents: i32) -> i32 {
    cents / 25
}
fn calc_nickels(cents: i32) -> i32 {
    cents / 5
}
fn calc_total(pennies: i32, dimes: i32, quaters: i32, nickels: i32) -> i32 {
    calc_pennies(pennies) + calc_dimes(dimes) + calc_quaters(quaters) + calc_nickels(nickels)
}
fn main() {
    //The user should be able to give an input
    println!("Enter the amount of cents");
    let mut cents = String::new();
    std::io::stdin().read_line(&mut cents).expect("Failed to read line");
    let cents: i32 = cents.trim().parse().expect("Please type a number!");
    let pennies = calc_pennies(cents);
    let dimes = calc_dimes(cents);  
    let quaters = calc_quaters(cents);
    let nickels = calc_nickels(cents);
    let total = calc_total(pennies, dimes, quaters, nickels);
    println!("Pennies is {}", pennies);
    println!("Dimes is {}", dimes);
    println!("Quaters is {}", quaters);
    println!("Nickels is {}", nickels);
    println!("Total is {}", total);
}
