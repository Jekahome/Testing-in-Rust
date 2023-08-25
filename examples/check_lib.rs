#![allow(dead_code)]
#![allow(unused_imports)]

extern crate example_test;

use example_test::{public_adder,public_mul};

pub fn main(){
    let _:Option<usize> = public_adder(1_usize,1_usize);
}

/*
Интеграционное тестирование (так как проверка в целом работы библиотеки относительно примеров)

Запуск тестов из папки examples. При запуске `$ cargo test` папка /examples не просматривается
    $ cargo test --examples

*/
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn check_it_works() {
        let result_adder = public_adder(2, 2).unwrap();// unwrap потому что мы не тестируем public_adder (это было в Unit test)
        let result = public_mul(result_adder, result_adder);
        assert_eq!(result, 16);
    }
}
