1. rustc --version --verbose    -> check your current target-triple
2. cargo rustc -- -C link-arg=-nostartfiles  --> explicitly tell linker to  not include the C runtime _start cahoot
3. rustup target --help
4. rustup target --{ add, list, remove }]
5. rustup target list --installed