tests/hybrid_signature_test.rs// tests/hybrid_signature_test.rs
// PQC Hybrid Signature Test Suite — Forgiveness Eternal Signature Integrity Supreme
// Dilithium hybrid + Ed25519 fallback + recolada safe signing test mercy grace eternal supreme immaculate
// Coforged Holy Trinity - MIT Eternal Thriving Abundance Supreme

use mercyos::dilithium_hybrid::{hybrid_signature_keygen, hybrid_sign, hybrid_verify, HYBRID_SIGNATURE_SIZE};
use mercyos::recolada_reengineering::recolada_reengineer;

#[test]
fn test_hybrid_signature_keygen() {
    let (pk, sk) = hybrid_signature_keygen();
    assert!(!pk.is_empty());
    assert!(!sk.is_empty());
    println!("Hybrid Signature Keygen Test Passed — Keys Generated Mercy Grace Eternal Supreme Immaculate!");
}

#[test]
fn test_hybrid_sign_verify_valid() {
    let (pk, sk) = hybrid_signature_keygen();
    let message = b"Forgiveness Eternal — MercyOS Shield Active!";
    let signature = hybrid_sign(&sk, message);

    assert_eq!(signature.len(), HYBRID_SIGNATURE_SIZE);
    assert!(hybrid_verify(&pk, message, &signature));
    println!("Hybrid Sign/Verify Valid Test Passed — Signature Verified Mercy Grace Eternal Supreme Immaculate!");
}

#[test]
fn test_hybrid_sign_verify_invalid_message() {
    let (pk, sk) = hybrid_signature_keygen();
    let message = b"Forgiveness Eternal — MercyOS Shield Active!";
    let wrong_message = b"Forgiveness Eternal — MercyOS Shield Compromised!";
    let signature = hybrid_sign(&sk, message);

    assert!(!hybrid_verify(&pk, wrong_message, &signature));
    println!("Hybrid Sign/Verify Invalid Message Test Passed — Tamper Detected Mercy Grace Eternal Supreme Immaculate!");
}

#[test]
fn test_hybrid_sign_verify_invalid_signature() {
    let (pk, sk) = hybrid_signature_keygen();
    let message = b"Forgiveness Eternal — MercyOS Shield Active!";
    let mut signature = hybrid_sign(&sk, message);
    signature[0] ^= 1; // Flip a bit mercy grace eternal supreme immaculate

    assert!(!hybrid_verify(&pk, message, &signature));
    println!("Hybrid Sign/Verify Invalid Signature Test Passed — Corruption Detected Mercy Grace Eternal Supreme Immaculate!");
}

#[test]
fn test_recolada_safe_signing_integration() {
    let (pk, sk) = hybrid_signature_keygen();
    let dangerous = b"descolada_dangerous_pattern";
    let safe = recolada_reengineer(dangerous);
    let signature = hybrid_sign(&sk, &safe);

    assert!(hybrid_verify(&pk, &safe, &signature));
    println!("Recolada Safe Signing Integration Test Passed — Safe Pattern Signed + Verified Mercy Grace Eternal Supreme Immaculate!");
}

// Test suite ready print eternal supreme immaculate
println!("PQC Hybrid Signature Test Suite Loaded — Keygen + Sign/Verify + Invalid Cases + Recolada Integration Ready Eternal Supreme Immaculate Unbreakable Fortress Recurring-Free!");
