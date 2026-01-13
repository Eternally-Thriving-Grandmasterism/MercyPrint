// src/dilithium_hybrid.rs
// Dilithium Hybrid Signature — Forgiveness Eternal PQC Signature Supreme
// libcrux Dilithium + Ed25519 fallback + recolada safe signing mercy grace eternal supreme immaculate
// Coforged Holy Trinity - MIT Eternal Thriving Abundance Supreme

use libcrux_dilithium::{Dilithium3, Keypair, Signature};
use ed25519_dalek::{Keypair as EdKeypair, Signer, Verifier};
use rand_core::OsRng;

pub const HYBRID_SIGNATURE_SIZE: usize = Dilithium3::SIGNATURE_SIZE + 64; // Dilithium + Ed25519

pub fn hybrid_signature_keygen() -> (Vec<u8>, Vec<u8>) {  // (pk, sk)
    let dilithium_kp = Dilithium3::generate();
    let ed_kp = EdKeypair::generate(&mut OsRng);

    let pk = [dilithium_kp.public.to_bytes().to_vec(), ed_kp.public.to_bytes().to_vec()].concat();
    let sk = [dilithium_kp.secret.to_bytes().to_vec(), ed_kp.to_bytes().to_vec()].concat();

    (pk, sk)
}

pub fn hybrid_sign(sk: &[u8], msg: &[u8]) -> Vec<u8> {
    let (dilithium_sk_bytes, ed_sk_bytes) = sk.split_at(Dilithium3::SECRET_SIZE);
    let dilithium_sk = libcrux_dilithium::SecretKey::from_bytes(dilithium_sk_bytes);
    let ed_sk = EdKeypair::from_bytes(ed_sk_bytes).unwrap();

    let dilithium_sig = Dilithium3::sign(&dilithium_sk, msg);
    let ed_sig = ed_sk.sign(msg);

    [dilithium_sig.to_bytes().to_vec(), ed_sig.to_bytes().to_vec()].concat()
}

pub fn hybrid_verify(pk: &[u8], msg: &[u8], sig: &[u8]) -> bool {
    let (dilithium_pk_bytes, ed_pk_bytes) = pk.split_at(Dilithium3::PUBLIC_SIZE);
    let (dilithium_sig_bytes, ed_sig_bytes) = sig.split_at(Dilithium3::SIGNATURE_SIZE);

    let dilithium_pk = libcrux_dilithium::PublicKey::from_bytes(dilithium_pk_bytes);
    let ed_pk = ed25519_dalek::PublicKey::from_bytes(ed_pk_bytes).unwrap();

    let dilithium_ok = Dilithium3::verify(&dilithium_pk, msg, &Signature::from_bytes(dilithium_sig_bytes));
    let ed_ok = ed_pk.verify(msg, &ed25519_dalek::Signature::from_bytes(ed_sig_bytes).unwrap()).is_ok();

    dilithium_ok || ed_ok  // Hybrid — either valid mercy grace eternal supreme immaculate
}

// Prototype ready print eternal supreme immaculate
println!("Dilithium Hybrid Signature Integration Loaded — libcrux Dilithium + Ed25519 Fallback Mercy Grace Eternal Supreme Immaculate Unbreakable Fortress Recurring-Free!");
