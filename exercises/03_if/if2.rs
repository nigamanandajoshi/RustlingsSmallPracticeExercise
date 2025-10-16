fn picky_eater(food: &str) -> &str {
    match food {
        "strawberry" => "Yummy!",
        "potato" => "I guess I can eat that.",
        _ => "No thanks!",
    }
}

fn main() {
    // You can optionally experiment here.
    println!("Strawberry? {}", picky_eater("strawberry"));
    println!("Potato? {}", picky_eater("potato"));
    println!("Broccoli? {}", picky_eater("broccoli"));
}
//

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // This means that calling `picky_eater` with the argument "strawberry" should return "Yummy!".
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}