fn main() {
    let mut my_number = 2;
    let num_ref = &mut my_number;

    // 2 rules with mutable references
    // 1. as many immutable references as you want
    // 2. Only one mutable references and can't have it together with immutable references

    *num_ref += 10;
    println!("Hello, world! {}", my_number);

    /*
    let mut hmm = 2;
    let hmm2 = &hmm;
    let hmm3 = &mut hmm;
    *hmm3 += 2;
    println!("hmm2 {}", hmm2);
     */

    let country = String::from("Rust");
    let country_ref = &country;
    let country = 2;
    println!("hmm {} {}", country, country_ref);
}
