![CI][1]
![Rust toolchain support][2]

**WARNING**: unstable API, may break at any time
```rust
use crew::{assemble_crew, CrewError}; 

// positive case with one worker running function f1
fn f1() -> Result<i32, CrewError> {
    Ok(42)
}
let val1 = assemble_crew::<i32>(1, f1).await;
assert_eq!(val1.unwrap(), Some(42));

// negative case with one worker running function f2
fn f2() -> Result<i32, CrewError> {
    Err(CrewError::WorkerFailure("I just can't"))
}
let val2 = assemble_crew::<i32>(1, f2).await;
assert_eq!(val2.is_err(), true);
```

The failure in the second case triggers a nice breadcrumb:
```console
Crew task interrupted by worker 9c30809e-5710-4420-8256-17a6a8a6d669. Cause: Worker reports a failure: I just can't
```

© 2020 Damien Stanton

See LICENSE for details.

[1]: https://github.com/damienstanton/crew/workflows/CI/badge.svg
[2]: https://img.shields.io/badge/Rust%20toolchain-stable-%23DEA484?style=plastic&logo=rust