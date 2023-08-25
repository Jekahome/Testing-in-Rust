// $ cargo run --bin parse_result "id:000000,sig:06,src:000000,time:210,execs:27,op:havoc,rep:7"
// $ paste fuzz_out/default/crashes/id* | sed 's/\t/\n/g'
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = std::env::args().collect(); 

    if args.len() > 1{
        let file_fuzzy_crashes = args[1].clone();
 
        let fuzzy_rezult:Vec<u8> = std::fs::read(format!("fuzz_out/default/crashes/{}",file_fuzzy_crashes))?;
        
        //for i in 0..fuzzy_rezult.len(){ print!("{:?},",u8::from_be_bytes([fuzzy_rezult[i]]));}
 
        let var_usize = usize::from_le_bytes(TryFrom::try_from(fuzzy_rezult[..8].to_vec()).unwrap());
        println!("\nFail fuzzy for usize:{}",var_usize);// Fail fuzzy for usiz:16276538888567202304

    }
    Ok(())
}