pub use add_calculate::{public_adder,public_adder_fail};
pub mod add_calculate {
    pub fn public_adder(left: usize, right: usize) -> Option<usize> {
        let (res,is_overflowing) = left.overflowing_add(right);
        if is_overflowing {
            return None;
        }
        Some(res)
    }
    pub fn public_adder_fail(left: usize, right: usize) -> Option<usize> {
        Some(left + right)
    }
    fn private_adder(left: usize, right: usize) -> Option<usize> {
        let (res,is_overflowing) = left.overflowing_add(right);
        if is_overflowing {
            return None;
        }
        Some(res)
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
            let result = private_adder(2, 2);
            assert_eq!(result, Some(4));
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
        let result = public_adder(2, 2);
        assert_eq!(result, Some(4));
    }

    /*
        Благодаря fuzzy мы обнаружили fail значением usize 16276538888567202304 
        Можем проверить
     */
    #[test]
    #[should_panic]
    fn fuzzy_help_fail_add(){
        let fuzzy:usize = 16276538888567202304;
        let _ = public_adder_fail(fuzzy, fuzzy);
    }
}

/*

    Unit тестирование

    Property based testing (Тестирование свойств)
*/
#[cfg(test)]
mod tests_prop {
    use super::*;
    use proptest::prelude::*;
    /*
    Возможная ошибка если убрать проверку переполнения:

    Выводит в консоль и в файл proptest-regressions/add.txt
        thread 'add::tests_prop::it_works_prop' panicked at 'Test failed: attempt to add with overflow.
        minimal failing input: a = 9343330405577739870, b = 9103413668131811746
     */
    proptest! {
        #[test]
        fn it_works_prop(a in usize::MIN..=usize::MAX, b in usize::MIN..=usize::MAX) {
            // возможный диапазон входов включен в параметры
            let _ = public_adder(a, b);
        }
    }
}