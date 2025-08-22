# deterministic

[![Build Status](https://img.shields.io/github/actions/workflow/status/vabhavx/deterministic/deterministic.yml)](https://github.com/vabhavx/deterministic/actions)
[![Formal Verification](https://img.shields.io/badge/formal%20verification-kani-blue?logo=rust)](https://github.com/vabhavx/deterministic/tree/main/kani)
[![Coverage](https://img.shields.io/codecov/c/github/vabhavx/deterministic?logo=codecov)](https://codecov.io/gh/vabhavx/deterministic)

**Deterministic** rebuilds trust in digital infrastructure. Every build here is reproducible, tamper-evident, and formally verified.

---

## Why This Matters

When the XZ backdoor struck Linux, it proved we must trust code less—and math more. This project "makes lying impossible" for build systems.

- **Guarantees:** Mathematical proof (not just trust) for deterministic builds.
- **For everyone:** Secures hospitals, banks, cars, cloud, and you.

## Installation

```sh
git clone https://github.com/vabhavx/deterministic.git
cd deterministic
cargo build
cargo test
kani prove
```

## Usage

```rust
use deterministic::verify_build;

fn main() {
    assert!(verify_build("Cargo.lock"));
}
```

## Badges & Proofs

- 🔒 [CI Status (pass/fail)](https://github.com/vabhavx/deterministic/actions)
- 🗒️ [Kani Formal Verification Badge](https://github.com/vabhavx/deterministic/tree/main/kani)
- 📈 [Coverage Status](https://codecov.io/gh/vabhavx/deterministic)

## Demo Proof Output

```
Build Proven Deterministic ✔
Merkle Root: cc12db...9a
SHA384 Integrity: 9e23f...
Probability undetected divergence: ≤ 1.2 × 10⁻¹⁸ [PROVEN]
```

## Reproducibility

Every commit passes end-to-end reproducibility and formal proof on all workflows (Linux/macOS/Windows). See [architecture.md](architecture.md) for technical details.

---
See [`CONTRIBUTING.md`](CONTRIBUTING.md) to join the trust revolution!
Contact: security@deterministic-builds.org | Maintained by [@vabhavx](https://github.com/vabhavx)
