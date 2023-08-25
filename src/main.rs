use example_test::{public_adder,public_mul};

fn main(){
    let args: Vec<String> = std::env::args().collect(); 
    if args.len() > 0{
        print!("args:{}",args[1]);
    }
    let _res:Option<usize> = public_adder(1_usize,1_usize);
    let _res:usize = public_mul(1_usize,1_usize);
}