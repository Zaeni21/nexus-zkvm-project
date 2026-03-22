use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};

const PACKAGE: &str = "guest";

fn main() {
    println!("--- NEXUS ZKVM FIBONACCI ENGINE ---");
    println!("Compiling guest program...");
    
    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> = Stwo::compile(&mut prover_compiler)
        .expect("failed to compile guest program");

    let elf = prover.elf.clone();

    println!("Proving execution of vm (Stwo Prover)...");
    let (view, proof) = prover.prove().expect("failed to prove program");

    println!(">>>>> Logging");
    let logs = view.logs().expect("failed to retrieve debug logs").join("");
    println!("{}", logs);
    println!("<<<<<");

    assert_eq!(
        view.exit_code().expect("failed to retrieve exit code"),
        nexus_sdk::KnownExitCodes::ExitSuccess as u32
    );

    print!("Verifying execution...");
    proof
        .verify_expected::<(), ()>(
            &(), 
            nexus_sdk::KnownExitCodes::ExitSuccess as u32,
            &(), 
            &elf, 
            &[], 
        )
        .expect("failed to verify proof");

    println!("  Succeeded!");
}
