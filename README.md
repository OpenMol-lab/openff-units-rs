# openff-units (Rust)

This crate is a Rust reimplementation of the portable part of
[OpenFF Units](https://github.com/openforcefield/openff-units). It provides a
serialisable `Unit`, `Quantity`, and `Measurement` API, the OpenFF default unit
and CODATA 2018 constant definitions, chemical element symbols and masses, and
conversion helpers for OpenMM-style `(magnitude, unit)` values.

```rust
use openff_units::{unit, Quantity};

let bond = Quantity::new(1.4, "angstrom")?;
let nanometers = bond.to("nanometer")?;
assert!((nanometers.value()? - 0.14).abs() < 1e-12);

let force_constant = unit().quantity(2000.0, "kilocalories_per_mole / angstrom**2")?;
```

The bundled `data/defaults.txt` and `data/constants.txt` are retained in their
Pint-compatible form and are available through the `openff_units::data` module.
See the API documentation for the complete registry and conversion surface.

## Upstream Source

This Rust implementation is ported from the upstream OpenFF Units project:

- Repository: <https://github.com/openforcefield/openff-units>
- Latest migrated upstream commit: [`3700032ad88ecb84166231e86ae3594aa15641e2`](https://github.com/openforcefield/openff-units/commit/3700032ad88ecb84166231e86ae3594aa15641e2)

The commit ID above identifies the latest `main` revision used as the migration
reference.
