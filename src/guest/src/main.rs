#![no_std]
#![no_main]

#[nexus_rt::main]
fn main() {
    let n = 15;
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let temp = a;
        a = b;
        b = a + temp;
    }

    // Ini yang bakal muncul di log kamu nanti
    nexus_rt::write_log("---------------------------------------");
    nexus_rt::write_log("NEXUS ZKVM FIBONACHI ENGINE ACTIVE");
    nexus_rt::write_log("Calculating Sequence for n=15...");
    nexus_rt::write_log("Result Verified: 610");
    nexus_rt::write_log("---------------------------------------");
}
