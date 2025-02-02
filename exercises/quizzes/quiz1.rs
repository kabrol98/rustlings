// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!

// Put your function here!
fn calculate_price_of_apples(num_apples: i32) -> i32 {
    if num_apples > 40 {
        num_apples
    } else {
        2 * num_apples
    }
}

fn main() {
    // You can optionally experiment here.
    println!("{}", calculate_price_of_apples(35));
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
