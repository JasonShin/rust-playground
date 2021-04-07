/*
// Following won't work since return_str() is the owner of country
fn return_str() -> &'static str {
    let country = String::from("Australia");
    let country_ref = &country;
    country_ref
}
*/

fn return_str() -> String {
    let country = String::from("Australia");
    country
}

fn main() {
    // OWNERSHIP = who owns a value
    // Rust only wants 1 owner at a time
    let country = String::from("Korea");

    let ref_one = &country;
    let ref_two = &country;

    println!("{} {} {}", country, ref_one, ref_two);

    let returned_country_ref = return_str();
    println!("{}", returned_country_ref)
}
