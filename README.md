# Serde URL Params

[![crates-badge]][crates-url]
[![docs-badge]][docs-url]
[![license-badge]][license]
[![ci-badge]][ci-url]

[crates-badge]: https://img.shields.io/crates/v/serde-url-params.svg
[crates-url]: https://crates.io/crates/serde-url-params
[docs-badge]: https://docs.rs/serde_url_params/badge.svg
[docs-url]: https://docs.rs/serde_url_params
[license-badge]: https://img.shields.io/crates/l/serde-url-params.svg
[license]: #license
[ci-badge]: https://github.com/boxdot/serde-url-params-rs/workflows/rust/badge.svg
[ci-url]: https://github.com/boxdot/serde-url-params-rs/actions

Serialization of URL parameters from Rust structs.

## Example

```rust
#[derive(Debug, Serialize)]
enum Filter { New, Registered, Blocked }

#[derive(Debug, Serialize)]
struct Params {
    cursor: Option<usize>,
    per_page: Option<usize>,
    username: String,
    filter: Vec<Filter>,
}

let params = Params {
    cursor: Some(42),
    per_page: None,
    username: String::from("boxdot"),
    filter: vec![Filter::New, Filter::Blocked],
};
assert_eq!(
    serde_url_params::to_string(&params).unwrap(),
    "cursor=42&username=boxdot&filter=New&filter=Blocked"
);
```

## License

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this document by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
