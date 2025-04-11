## 0.5.0 (2025-04-11)

A rename from `0.4.2` as it requires Rust `>=1.85.0`

## 0.4.2 (2025-04-10) (yanked)

### What's Changed

- Avoid `decode` function to uppercase the input and lead to different result
- Update to 2024 edition

## 0.4.1 (2025-01-28)

### What's Changed

- Upgrade `rand` dependency from version `~0.8.5` to `~0.9.0`

## 0.4.0 (2024-10-27)

### What's Changed

- `encode` function accept more types of input now
- `decode` function accept more types of input now
- `generate` function accept more types of input now
- `verify` function accept more types of input now
- `system_time_to_timestamp` function accept more types of input now
- `char_list` in `RowIDWithConfig` accept more types of input now

## 0.3.0 (2024-10-14)

### Breaking Changes

- Add `base` module, and move related functions into it
- Add `with_config` module, and move related functions into it
- Add `time` module, and move related functions into it

## 0.2.0 (2024-10-13)

### Breaking Changes

- Changes in accepted value type of `decode`:
    - `String` => `&str`
- Changes in accepted value type of `verify`:
    - `String` => `&str`
- Changes in accepted value type of `char_list` in `RowIDWithConfig`:
    - `String` => `&str`
- Merge different error messages into `RowIDError` enum

### What's New

- Add different derives for different structs

### What's Changed

- Updates in documentation

## 0.1.1 (2024-08-04)

### What's Changed

- Update description
- Update type declaration

## 0.1.0 (2024-06-22)

First release
