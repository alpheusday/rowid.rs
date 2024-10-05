# RowID

A time-based unique ID solution.

## Installation

To install this package, run the following command:

```bash
cargo add rowid
```

## Quick Start

Create an ID with the following code:

```rust
use rowid::rowid;

let id: String = rowid();
```

Or start a customization with the following code:

```rust
use rowid::{RowIDWithConfig, RowIDWithConfigResult};

let rwc: RowIDWithConfigResult = RowIDWithConfig::new()
    .char_list("0123456789ABCDEFGHJKMNPQRSTVWXYZ".to_string())
    .randomness_length(22)
    .done()
    .unwrap();

let id: String = rwc.rowid();
```

## License

This project is MIT licensed, you can find the license file [here](./LICENSE).
