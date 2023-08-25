#![allow(dead_code)]
#![allow(unused_must_use)]

pub mod add;
pub mod mul;
pub mod email;
pub mod faker;
pub use add::add_calculate::{public_adder,public_adder_fail};
pub use mul::mul_calculate::public_mul;

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

/*
    Unit testing stdout

    Snapshot testing
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn stdout_find_a_match() {
        let mut result = Vec::new();
        find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }

    /*
        Snapshot testing
        
        1. $ cargo test tests::stdout_insta_find_a_match -- --ignored
        2. $ cargo insta review
        3. $ cargo test tests::stdout_insta_find_a_match -- --ignored
    */
    #[test]
    #[ignore]
    fn stdout_insta_find_a_match() {
        let mut result = Vec::new();
        find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
        insta::assert_yaml_snapshot!(result);
    }
}

