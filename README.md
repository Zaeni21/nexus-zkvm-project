# Nexus zkVM: Hello World Project

This repository contains a basic implementation of a Zero-Knowledge application using the **Nexus SDK**. It demonstrates how to write a "Guest" program in Rust, compile it to **RISC-V**, and generate a ZK proof on a **Host** machine.

## 🚀 Project Overview

- **Host:** The main driver that executes the zkVM and generates proofs.
- **Guest:** The ZK-proven logic (located in `src/guest`) that runs inside the Nexus Virtual Machine.
- **Architecture:** RISC-V 32-bit (riscv32im-unknown-none-elf).

## 🛠 Prerequisites

To run this project, you need:
- **Rust Nightly** (specifically `nightly-2025-05-09`)
- **Nexus SDK** installed
- **RISC-V Target:** `riscv32im-unknown-none-elf`

## 🏃 How to Run

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Zaeni21/nexus-zkvm-project.git
   cd nexus-zkvm-project
   ```

2. **Run the Prover (Release Mode):**
   ```bash
   cargo run -r
   ```

## 📈 Technical Details
This project is built as part of the **Nexus Network** ecosystem. It utilizes the `no_std` environment for the guest program to ensure compatibility with the zkVM constraints.

---
**Developed by [Zaeni21](https://github.com/Zaeni21)** 
*Building the future of Verifiable Computing.*
