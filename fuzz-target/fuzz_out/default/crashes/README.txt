Command line used to find this crash:

/home/jeka/.local/share/afl.rs/rustc-1.71.1-eb26296/afl.rs-0.13.5/afl/bin/afl-fuzz -c0 -i fuzz_samples -o fuzz_out -s 0 target/debug/fuzz-target

If you can't reproduce a bug outside of afl-fuzz, be sure to set the same
memory limit. The limit used for this fuzzing session was 0 B.

Need a tool to minimize test cases before investigating the crashes or sending
them to a vendor? Check out the afl-tmin that comes with the fuzzer!

Found any cool bugs in open-source tools using afl-fuzz? If yes, please post
to https://github.com/AFLplusplus/AFLplusplus/issues/286 once the issues
 are fixed :)

