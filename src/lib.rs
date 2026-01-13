// src/lib.rs
// MercyOS Core Library — Forgiveness Eternal From-Scratch no_std Rust Post-Quantum Cryptography Fortress
// uniffi Swift + Python bindings + Dilithium Hybrid Signatures + ML-KEM Hybrid KEM + MercyPrint one-command minting mercy grace eternal supreme immaculate

#![no_std]
extern crate alloc;

mod ml_kem_hybrid;
mod dilithium_hybrid;
mod recolada_reengineering;
mod mercy_print; // MercyPrint module mercy grace eternal supreme immaculate

pub use ml_kem_hybrid::{hybrid_keygen as hybrid_kem_keygen, hybrid_encaps, hybrid_decaps, HYBRID_SHARED_SECRET_SIZE};
pub use dilithium_hybrid::{hybrid_signature_keygen, hybrid_sign, hybrid_verify, HYBRID_SIGNATURE_SIZE};
pub use recolada_reengineering::recolada_reengineer;
pub use mercy_print::print; // MercyPrint one-command export mercy grace eternal supreme immaculate

uniffi::include_scaffolding!("mercyos");

// Existing exports + MercyPrint mercy grace eternal supreme immaculate
#[uniffi::export]
pub fn print_app(app_name: String) {
    print(&app_name);
}

// ... existing exports ...

println!("MercyOS Core Library Loaded + MercyPrint Integration Ready — One-Command Mercy App Minting Eternal Supreme Immaculate Unbreakable Fortress Recurring-Free!");
