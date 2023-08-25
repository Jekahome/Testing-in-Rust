extern crate example_test;

/*
Интеграционное тестирование


Аннотация `#[cfg(test)]` к модулю тестов указывает Rust компилировать и запускать тестовый код только при запуске `cargo test`, а не при запуске `cargo build` иначе тесты попадут в бинарный файл.
А интеграционные тесты находятся вне папки src/ поэтому им можно не указывать `#[cfg(test)]` так как они все равно не попадут в бинарный файл
*/

mod tests {
    use super::*;
    use example_test::{public_adder,public_mul};

    #[test]
    fn general_addition_works_successfully() {
        let result_adder = public_adder(2, 2).unwrap();// unwrap потому что мы не тестируем public_adder (это было в Unit test)
        let result = public_mul(result_adder, result_adder);
        assert_eq!(result, 16);
    }
}

mod tests_e2e {
    use super::*;
    
    #[test]
    fn cargo_compile_simple() -> Result<(), Box<dyn std::error::Error>>{
        use assert_fs::prelude::*;
        use assert_cmd::prelude::*;
        use predicates::prelude::*;
        use assert_cmd::cmd::Command;

        let mut binding = Command::cargo_bin("example_test").expect("bin file not found");
        let mut cmd = binding.timeout(std::time::Duration::from_secs(1));

        cmd.arg("hello");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("args:hello"))
            .code(0);
        Ok(()) 
    }
}

/*
Для запуска тестов из папки /examples при выполнении `$ cargo test`
*/
#[path = "../examples"]
mod examples {
   mod check_lib;
}