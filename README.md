# WIP: Serde URL Params [![Build Status](https://travis-ci.org/boxdot/serde-url-params-rs.svg?branch=master)](https://travis-ci.org/boxdot/serde-url-params-rs) [![Build status](https://ci.appveyor.com/api/projects/status/634yhym9f5cpb2qc/branch/master?svg=true)](https://ci.appveyor.com/project/boxdot/serde-url-params-rs/branch/master)

Serialization of URL parameters from Rust structs.

```toml
[dependencies]
serde_url_params = "0.1"
```

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

## WIP

* [x] Do not use Error::custom for unsupported serialization errors.
* [ ] Escape url params in strings.
* [ ] Expand tests.
* [ ] Add docs.
* [ ] Add support for maps with arbitrary keys.

## License

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this document by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
