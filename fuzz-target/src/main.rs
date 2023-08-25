
extern crate afl;
extern crate example_test;

//https://crates.io/crates/afl
//https://crates.io/crates/evcxr

// $ cd example_test

// # Создание бинарного subroject
// $ cargo new fuzz-target


//# Создание начального входного значения в бинарном виде для fuzzing при запуске build.rs отработает
//Должна быть папка для входных данных example_test/fuzz-target/fuzz_samples

// # Сборка afl
// $ cd fuzz-target
// $ cargo afl build

// # Запуск fuzzing
// $ cd /sys/devices/system/cpu
// $ sudo echo performance | sudo tee cpu*/cpufreq/scaling_governor
// $ AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES=1 cargo afl fuzz -i fuzz_samples -o fuzz_out -s 0 target/debug/fuzz-target

/*

Анализ вывода терминала:

saved crashes - сколько для нас сохранил fails для просмотра
total crashes - сколько всего было найдено fails
total execs - сколько всего перебрал вариантов

Анализ вывода паки fuzz_out/default/crashes
$ ls fuzz_out/default/crashes
>>> id:000000,sig:06,src:000000,time:210,execs:27,op:havoc,rep:7
>>> README.txt

Просмотр варианта приведшего к fail
$ cargo run --bin parse_result "id:000000,sig:06,src:000000,time:210,execs:27,op:havoc,rep:7"

Повтор fail результата:
/home/jeka/.local/share/afl.rs/rustc-1.71.1-eb26296/afl.rs-0.13.5/afl/bin/afl-fuzz -c0 -i fuzz_samples -o fuzz_out -s 0 target/debug/fuzz-target

*/

 

fn main() {
  // Нам просто нужно вызвать нашу функцию внутри макроса
  /* 
    // Входные данные всегда raw bytes
    afl::fuzz!(|data: &[u8]| {
        if let Ok(value) = data.try_into(){
            let n = usize::from_le_bytes(value);
            // Вызовите функцию. Проигнорируйте результат, потому что нас интересуют сбои
            let _ = example_test::public_adder_fail(n,n);
        }
    });
  */
    // Благодапя `crate arbitrary` входные данные из raw bytes поступают уже приведенные к нужному типу Rust
    afl::fuzz!(|data: usize| {
        if let Ok(value) = data.try_into(){
            let _ = example_test::public_adder_fail(value,value);
        }
    });
}