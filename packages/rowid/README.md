# RowID

A time based unique ID solution.

## Quick Start

You may create a RowID with the following code:

```rust
use rowid::rowid;

let id: String = rowid();
```

Or customize the RowID with the following code:

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

This project is MIT licensed, you can find the license file [here](https://github.com/alpheustangs/rowid.rs/blob/main/LICENSE).
