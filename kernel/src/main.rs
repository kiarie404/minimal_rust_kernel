#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[panic_handler]
fn custom_panic(_info: &PanicInfo) -> !{
    loop{};
}


#[no_mangle]
pub extern "C" fn _start(){
    loop{};
}

// We have defined our new custom target for our compiler.
// WE will use the universal LLD linker to create an executable for our custom target.
// we need to compile  the core library for our custom target.
// the supported targets in Rust already come with the pre-compiled core library suitable for them
// since we have declared a new target, we have to recompile core for this new target.


// Now the rust nightly feature : "build-std" allows us to recompile std, core and other standard library crates on demand, instead of using the precompiled versions shipped with the Rust installation
// To use the feature, we need to create a cargo configuration file at .cargo/config.toml



