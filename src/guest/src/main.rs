#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::print;

#[nexus_rt::main]
fn main() {
    let value1 = 10;
    let value2 = 20;
    let result = value1 + value2;
    
    // We print the result back to the Host terminal
    if result == 30 {
        print!("ZK Calculation Successful: 10 + 20 = 30\n");
    } else {
        print!("Something went wrong!\n");
    }
}
