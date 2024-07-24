## v0.2.3 (2024-07-24)

## v0.2.2 (2024-05-27)

### Fix

- **deps**: update dependency @splinetool/runtime to v1.4.1
- **deps**: update rust crate serde to v1.0.203
- **deps**: update rust crate dioxus-logger to v0.5.1

## v0.2.1 (2024-05-04)

### Fix

- **version**: Set correct version

## v0.2.0 (2024-05-04)

### BREAKING CHANGE

- The old API where position and scale are set with SPEVector3 will no longer work

### Feat

- **spline**: display none on loading scene
- **speobject**: Change SPEObject API to be more handy
- **runtime**: Return `SPEObject`s directly
- **example**: Update example
- **runtime**: Refactor `Application`, change its name to `SplineApplication`

### Fix

- **examples**: Readd web.app to examples and bump dioxus-spline
- **deps**: update rust-wasm-bindgen monorepo
- **deps**: update rust crate serde to 1.0.200
- **deps**: update rust crate tracing to 0.1.40
- **deps**: update rust crate dioxus to 0.5.1
- **deps**: update dependency @splinetool/runtime to v1.2.5

## v0.1.0 (2024-05-03)

### Feat

- **cargo.toml**: add metadata to Cargo.toml
- event listeners, SPEObject and others
- I think I got SplineEvent
- attempt to add external listeners on spline events

### Fix

- **pages**: Move [web.app] from Cargo.toml to Dioxus.toml (its 6am i am so cooked)
- **pages**: Now web.app is set correctly
- **cargo.toml**: exclude github from package

### Refactor

- **spline**: Remove unnecessary futures
