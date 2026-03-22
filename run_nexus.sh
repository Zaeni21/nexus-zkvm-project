#!/bin/bash
# Pastikan di folder yang benar
cd /root/nexus-host

# Ambil environment Rust supaya script tahu di mana 'cargo' berada
source $HOME/.cargo/env

echo "--- Script Started: $(date) ---" > nexus_output.log

while true; do
    echo "[Sun Mar 22 02:28:13 AM UTC 2026] Memulai Proving..." >> nexus_output.log
    
    # Jalankan perintah langsung tanpa 'rustup run' jika sudah di nightly
    # Ini jauh lebih ringan dan jarang crash
    cargo run -r >> nexus_output.log 2>&1
    
    echo "[Sun Mar 22 02:28:13 AM UTC 2026] Selesai. Istirahat 5 detik..." >> nexus_output.log
    sleep 5
done
