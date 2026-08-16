# separation-bounds

[![CI](https://github.com/kitt159/separation-bounds/actions/workflows/ci.yml/badge.svg)](https://github.com/kitt159/separation-bounds/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/separation-bounds.svg)](https://crates.io/crates/separation-bounds)
[![docs.rs](https://docs.rs/separation-bounds/badge.svg)](https://docs.rs/separation-bounds)
[![msrv](https://img.shields.io/crates/msrv/separation-bounds.svg)](https://crates.io/crates/separation-bounds)
[![license](https://img.shields.io/crates/l/separation-bounds.svg)](#license)

Constructive **separation bounds**: exact zero and sign **proofs** for arithmetic expressions.

## Roadmap

- [ ] BFMSS bound with expressions represented as trees
- [ ] ability to encode expressions as DAGs to get better bounds
- [ ] automatic transformation into DAG
- [ ] simpler expression construction using operators or macros
- [ ] power node, instead of multiple multiplications
- [ ] dyadic rational node, instead of division for better handling of float constants
- [ ] BFMSS[k] optimization
- [ ] Li-Yap bound
- [ ] heuristics for picking bound type
- [ ] `RootOf` / diamond operator, to represent every real algebraic number
- [ ] DAG rewrite rules to get better bounds
- [ ] return negative / zero / positive, instead of the bit count
- [ ] optimize evaluation of easy cases
- [ ] ...

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
 * MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
