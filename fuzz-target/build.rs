
fn main(){
    // Создание начального входного значения в бинарном виде
    let _ = std::fs::write("fuzz_samples/input_file_binaries",[0, 0, 0, 0, 0, 0, 0, 0] /*2usize.to_le_bytes()*/);
    //  $ echo -n "2" > fuzz_samples/input_file_binaries
}