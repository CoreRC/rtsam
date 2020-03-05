# RTSAM

![crate](https://img.shields.io/crates/v/rtsam.svg)

Real Time Smoothing and Mapping (RTSAM) in Rust.

# Development Plan

- [x] Standard Lie algebra in Rust
- [ ] Working simple factor graph generation
- [ ] Working LLVM-based JIT for CPU/GPU targets (WIP branch `dev-jit`)
- [ ] Working symbolic evaluation and autograd
- [ ] Design a continous storage strategy for factors
- [ ] Dynamic scheduling of JIT'ed code fragments
- [ ] GPU LM/GS Kernels with JIT'ed lazy evaluation

# Warning

This work is a WIP.

# LICENSE

BSD 3-Clause

Copyright (c) 2020, Fan Jiang

Code from the nalgebra project: Copyright (c) 2013, Sébastien Crozet