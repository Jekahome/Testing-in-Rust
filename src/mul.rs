pub use mul_calculate::public_mul;
pub mod mul_calculate {
    pub fn public_mul(left: usize, right: usize) -> usize {
        left * right
    }

    fn private_mul(left: usize, right: usize) -> usize {
        left * right
    }

    /*
        Unit тестирование

        Тестирование приватных методов
    */
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn intern_addition_works_successfully() {
            let result = private_mul(2, 2);
            assert_eq!(result, 4);
        }
    }  
}


/*
    Unit тестирование

    Аннотация `#[cfg(test)]` к модулю тестов указывает Rust компилировать и запускать тестовый код только при запуске `cargo test`, 
    а не при запуске `cargo build`
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = public_mul(2, 2);
        assert_eq!(result, 4);
    }
}