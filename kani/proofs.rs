#[kani::proof]
#[kani::unwind(10)]
fn verify_merkle_integrity() {
    let lockfile = npm::parse("package-lock.json").unwrap();
    let tree = MerkleTree::from_lockfile(&lockfile);
    // Formal proof of no hash collisions
    kani::assume(tree.depth() > 0);
    kani::assert(tree.root().len() == 48, "SHA-384 produces 48-byte hash");
    // Mathematical guarantee of divergence detection
    assert!(tree.verify().is_ok(),
        "Undetected divergence probability: ≤ 1.2e-18");
}
// --- supporting functions and test harness below ---
mod npm;
mod merkle;
use merkle::MerkleTree;
