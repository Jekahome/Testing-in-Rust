#![allow(unused_imports)]

extern crate fake;

use fake::{Dummy, Fake, Faker};
use fake::locales::EN;
use fake::faker::name::en::FirstName;

use faker::*;
pub mod faker{
    use super::*;

    #[derive(Debug, Dummy)]
    pub struct Foo {
        #[dummy(faker = "1000..2000")]
        order_id: usize,
        #[dummy(faker = "FirstName()")]
        first_name: String,
        #[dummy(expr = "\"Huan\".into()")]
        last_name: String,
        #[dummy(expr = "true")]
        paid: bool,
    }

    pub fn foo(foo:Foo)->bool{
        foo.paid
    }
}

/*

    Unit тестирование

    Generating fake data 
*/
#[cfg(test)]
mod tests_prop {
    use super::*;
  
    #[test]
    fn it_works_fake() {

        let f: Foo = Faker.fake();
        assert!(foo(f));
 
        // let first: String = fake::faker::name::raw::FirstName(EN).fake();
        // let last: String = fake::faker::name::raw::LastName(EN).fake();
    }
}