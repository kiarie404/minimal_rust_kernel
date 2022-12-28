// Making a free-standing binary
// To make a free standing library...
// 1. You have to drop the std library support
// 2. You have to use the core library
// 3. You have to define your own function that gets called when a panic happens. (The std panic function is OS dependent eg: it depends on the standard I/O and it uses stack unwinding )
// 4. You have to disable stack unwinding because it is OS-dependent 

// Explicitly declare that our code will not use the Standard Library
// The standard library is dependent on the Host Operating system and libC library 
// By default Rust programs summon std and its prelude by default
// By default, programs with the #![no-std] summon the core library and its prelude
#![no_std]
#![no_main] 

// PanicInfo is a structure that contains the file and line where the panic occurred.
// PanicInfo structure might contain the optional panic message
use core::panic::PanicInfo;



#[panic_handler]
// The panic_handler attribute is defined above the function that should be called by the compiler when a panic happens.
// The custom panic should have at least one argument -> THe panic info argument
// The custom panic should return the "never" type.
// The PanicInfo parameter contains the file and line where the panic happened and the optional panic message.b
// We reference PanicInfo immutably, we cannot change panic information; that wou;d be unprofessional.
fn custom_panic (_info: &PanicInfo) -> !{
    loop {  }
}


// A panic should be handled gracefully.
// 1. The error message should be returned to the relevant calling function. Let us call this function Function A.
// 2. The memory occupied by variables defined in the chain of functions called by A should be freed.
// Executing step 1 and 2 is collectively called Stack-unwinding.
// Stack UNwinding is dependent on operating_system-specific libraries.
// Rust programs invoke stack-unwinding by default. We need to change that for our free-standing library.
// To fix that, we modify cargo.toml by specifying some compiler settings under both dev and release profiles

// execution of a program.
// For most C and C++ programs, the true entry point is not main, it’s the _start function. This function ...
    // 1. Initializes the program runtime
    // 2. Invokes the program’s main function

// A runtime system sets up an environment for a program to run on
// The runtime may load the initial variables to the register.
// The runtime sets up and manages the stacks required by the program. For example the function_call stack
// the runtime interfaces the language functions with the hosts systems functions.

// A rust program that depends on the standard library uses both the C runtime and the Rust Runtime 
// Both runtimes are used to set up the environment for running the hello.r program
// When a rust program is ran, it calls the C runtime system called Crt0
    // - CRT0 sets up the stacks
    // - CRT0 calls the Rust runtime system by finding the "start" function of the Rust runtime system. The "start" function is definned as a Language item using the "#[lang = "start"]" attribute

// THe Rust runtime adds safety checks on the runtime environnmet.
// The Rust runtime then calls the "main" function of our program.

// Our freestanding library does not have access to both Crt0 and Rust_Runtime library;
    // Because the CRT0 is the default entry point of a program, we have to define our own custom entry point.
    // For our custom entry point to be the prefered entry point, we have to instruct the rust compiler to ignore the default entry point. we do this by  adding the attribute "#[no_main]" to our code.
 
// SRC: https://en.wikibooks.org/wiki/X86_Disassembly/Calling_Conventions
// we wil define our _start function using C Calling convention. The calling convention specifies how a function call in C or C++ is converted into assembly language.
// THe C calling convention will be used through out


// Our _start function does not get called, it gets invoked. Authority is transfered to it from either the Operating system or the bootloader.
// So this means that the _start function does not have a return value. It lacks the ability to return anything; even void.
// If it was an OS that had called our _start function, it would have been hornorable to make our _start function to call the _exit system call when our program ends.
// It it was a bootloader that had called our _start function, it would have been hornorable to make our _start function to shut down the machine

#[no_mangle] // export function name as-is
pub extern "C" fn _start() -> !{ // our function is diverging, it has no function to return to.
                                 // _start uses the C calling convention
    loop{}
}


// making an executable object file
// The compiler takes in Rust code and produces an Object file. An object file is a file with machine code.
// The object file might depend on other libraries. For example, a normal runtime depends on C runtime libraries.
// The object file produced by the compiler is not executable.
// The linker takes in many object files and their dependent library object files and links them together to form a single executable.
// THe linker might also output a unified library... it does not just output single executables.


// The problem with our rust program is that the Rust default linker creates an executable for the coders host system. Most host systems use the C runtime libraries
// Check your host system target-triple using the command "rustc --version --verbose"
// THe reason this is a problem is because by default, creating executables for many operating systems requires the C runtime. And in our case, we do not want the C runtime. Moreover, our code cannot access it.
// To solve this problem, we must make our linker to stop automatically linking the C runtime libraries. To stop it we can
    // 1. make the linker build for bare metal; ie. we give the compiler a triple-target that declares the operating system as "None". This way, the linker does not automatcaly link the C runtime libraries that we have no business with
    // 2. instruct the linker to explicitly ignore the C runtime files


// We take the bare-metal path we build for the ARM Corex M3 micro-controller whose triple target is : thumbv7em-none-eabihf