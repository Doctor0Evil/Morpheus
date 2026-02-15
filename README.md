# Quantum Neuromorph Hub

Multi-crate Rust workspace that unifies:

- Church-of-FEAR style neuromorphic governance and non-reversal rights.
- CEIM/CPVM eco-impact kernels for Phoenix water nodes.
- Five production-grade planners/schedulers and an EcoNet dashboard API.

All binaries are GitHub- and CI-ready, and crates integrate cleanly with external orchestrators and AI chat agents.

## Crates

- `governance-core`: Roles, multisig, DeedEvent audit log, reversal policy kernel.
- `neuromorph-sim`: Non-actuating envelope snapshots and mitigation reprojection.
- `ceim-kernel`: CEIM node kernel, mass-load and academic integral grammar.
- `cpvm-kernel`: Viability kernels and Lyapunov-style residuals for hardware.
- `phoenix-bridge`: CEIM–CPVM bridge daemon for Phoenix water feeds.
- `nitrate-mar-planner`: Corridor planner for nitrate MAR basins.
- `pfas-selector`: PFAS treatment selector with CEIM-XJ supreme operator.
- `cybo-intake-scheduler`: Intake scheduler for TDS and nitrate.
- `econet-dashboard`: REST API over CEIM shards for EcoNet Phoenix UI.

## Build

```bash
cargo build --workspace
cargo test --workspace
Run examples
bash
cargo run -p phoenix-bridge
cargo run -p econet-dashboard
text


```text
# quantum-neuromorph-hub/LICENSE
MIT License

Copyright (c) 2026

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

[full standard MIT text here]


  quantum-neuromorph-hub/
    Cargo.toml
    rust-toolchain.toml
    README.md
    LICENSE
    .gitignore
    .github/
      workflows/
        ci.yml
    aln/
      reversal_policy.aln
    crates/
      governance-core/
        Cargo.toml
        src/
          lib.rs
          roles.rs
          audit.rs
          decision.rs
      neuromorph-sim/
        Cargo.toml
        src/
          lib.rs
          envelope.rs
          mitigation.rs
          projector.rs
      ceim-kernel/
        Cargo.toml
        src/
          lib.rs
          ceim.rs
          mass_load.rs
          regulatory.rs
      cpvm-kernel/
        Cargo.toml
        src/
          lib.rs
          viability.rs
          lyapunov.rs
      phoenix-bridge/
        Cargo.toml
        src/
          main.rs
          config.rs
          feeds.rs
          state.rs
          shards.rs
      nitrate-mar-planner/
        Cargo.toml
        src/
          main.rs
          model.rs
          scheduler.rs
      pfas-selector/
        Cargo.toml
        src/
          main.rs
          design.rs
          supreme.rs
      cybo-intake-scheduler/
        Cargo.toml
        src/
          main.rs
          series.rs
          optimizer.rs
      econet-dashboard/
        Cargo.toml
        src/
          main.rs
          api.rs
          views.rs
          storage.rs
    web/
      econet-dashboard-ui/
        package.json
        tsconfig.json
        vite.config.mts
        src/
          main.ts
          api.ts
          components/
            App.ts
            NodeCard.ts
            CorridorMap.ts
            EcoImpactGauge.ts
