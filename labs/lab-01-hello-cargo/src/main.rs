/// Builds a greeting for the given name.
fn greeting(name: &str) -> String {
    format!("Hello, {name}! Welcome to rust-labs.")
}

fn main() {
    println!("{}", greeting("Fernando"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        assert_eq!(greeting("Ferris"), "Hello, Ferris! Welcome to rust-labs.");
    }
}
