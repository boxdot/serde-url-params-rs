//! # Serde URL Params
//!
//! This module provides a simple and flexible way for serializing data
//! structures into URL parameters strings.
//!
//! A data structure can be converted to such string by
//! [`serde_url_params::to_string`][to_string] function. There is also
//! [`serde_url_params::to_vec`][to_vec] which serializes to a `Vec<u8>` and
//! [`serde_url_params::to_writer`][to_writer] which serializes to any
//! `io::Write` such as a File or a TCP stream.
//!
//! ```rust
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! enum Filter {
//!     Horror,
//!     Comedy,
//!     Thriller,
//!     Drama,
//! }
//!
//! #[derive(Serialize)]
//! struct Options {
//!     year: u16,
//!     actors: Vec<String>,
//! }
//!
//! #[derive(Serialize)]
//! struct SearchRequest {
//!     film: String,
//!     per_page: Option<usize>,
//!     next: Option<usize>,
//!     filter: Vec<Filter>,
//!     #[serde(flatten)]
//!     options: Options,
//! }
//!
//! fn main() -> Result<(), serde_url_params::Error> {
//!     // Some data structure.
//!     let request = SearchRequest {
//!         film: String::from("Fight Club"),
//!         per_page: Some(20),
//!         next: None,
//!         filter: vec![Filter::Thriller, Filter::Drama],
//!         options: Options {
//!             year: 1999,
//!             actors: vec!["Edward Norton".into()],
//!         },
//!     };
//!
//!     // Serialize it to a URL parameters string.
//!     let p = serde_url_params::to_string(&request).unwrap();
//!     assert_eq!(
//!         p,
//!         "film=Fight+Club&per_page=20&filter=Thriller&filter=Drama&year=1999&actors=Edward+Norton"
//!     );
//!     Ok(())
//! }
//! ```
//!
//! Almost any type that implements Serde's `Serialize` trait can be serialized
//! this way. This includes the built-in Rust standard library types `Vec<T>`
//! as you can see in the above example, as well as structs or enums annotated
//! with `#[derive(Serialize)]`. However, there are exceptions, for which it is
//! not obvious how to serialize them into flat parameters list:
//!
//! * any simple top level value, since it does not have a parameter key, and
//! * any nested struct, since it is not obvious how to flatten it,
//! * any map, which is not flattened (i.e. annotated with `#[serde(flatten)]`).
//!
//! Further, any string is automatically URL encoded (or more precisely,
//! percentage encoded). Elements in `Vec`s are serialized as repeated
//! `key=value` pairs, where key is the field holding the vector. Newtype
//! variants and variant structs are flattened by omitting the name of the
//! variant resp. struct.
//!
//! [to_string]: ser/fn.to_string.html
//! [to_vec]: ser/fn.to_vec.html
//! [to_writer]: ser/fn.to_writer.html

#![deny(missing_docs)]

#[doc(inline)]
pub use self::error::{Error, Result};
#[doc(inline)]
pub use self::ser::{to_string, to_vec, to_writer, Serializer};

pub mod error;
pub mod ser;

#[cfg(test)]
mod tests {
    use super::to_string;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    enum Selection {
        A,
        B,
    }

    #[derive(Debug, Serialize)]
    struct Request {
        id: String,
        filter: Vec<String>,
        option: Option<String>,
        optional_filter: Option<Vec<String>>,
        select: Selection,
        select2: Vec<Selection>,
        num: Option<usize>,
        results: Vec<::std::result::Result<&'static str, &'static str>>,
    }

    #[test]
    fn test() {
        let request = Request {
            id: String::from("some_id"),
            filter: vec![String::from("filter1"), String::from("filter2")],
            option: None,
            optional_filter: Some(vec![String::from("filter3")]),
            select: Selection::A,
            select2: vec![Selection::A, Selection::B],
            num: Some(42),
            results: vec![Ok("pass"), Err("fail")],
        };
        let get_params = to_string(&request);
        assert!(get_params.is_ok());
        assert_eq!(
            get_params.unwrap(),
            "id=some_id&filter=filter1&filter=filter2&optional_filter=filter3&select=A&select2=A&select2=B&num=42&results=pass&results=fail"
        );
    }

    #[test]
    fn test_newtype_struct() {
        #[derive(Debug, Serialize)]
        struct NewType(usize);
        #[derive(Debug, Serialize)]
        struct Params {
            field: NewType,
        }
        let params = Params { field: NewType(42) };
        let url_params = to_string(&params);
        assert!(url_params.is_ok());
        assert_eq!(url_params.unwrap(), "field=42");
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_tuple() {
        #[derive(Debug, Serialize)]
        struct Params {
            field: (usize, &'static str, f32),
        }
        let params = Params {
            field: (42, "hello", 3.14),
        };
        let url_params = to_string(&params);
        assert!(url_params.is_ok());
        assert_eq!(url_params.unwrap(), "field=42&field=hello&field=3.14");
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_tuple_struct() {
        #[derive(Debug, Serialize)]
        struct TupleStruct(usize, &'static str, f32);
        #[derive(Debug, Serialize)]
        struct Params {
            field: TupleStruct,
        }
        let params = Params {
            field: TupleStruct(42, "hello", 3.14),
        };
        let url_params = to_string(&params);
        assert!(url_params.is_ok());
        assert_eq!(url_params.unwrap(), "field=42&field=hello&field=3.14");
    }

    #[test]
    fn test_struct() {
        #[derive(Debug, Serialize)]
        struct A {
            username: String,
        }
        #[derive(Debug, Serialize)]
        struct Params {
            field: A,
        }
        // top level struct is supported
        {
            let params = A {
                username: String::from("boxdot"),
            };
            let url_params = to_string(&params);
            assert!(url_params.is_ok());
            assert_eq!(url_params.unwrap(), "username=boxdot");
        }
        // nested struct is not supported
        {
            let params = Params {
                field: A {
                    username: String::from("boxdot"),
                },
            };
            let url_params = to_string(&params);
            assert!(url_params.is_err());
        }
    }

    #[test]
    fn test_struct_variant() {
        #[derive(Debug, Serialize)]
        enum StructVariant {
            A { username: String },
        }
        #[derive(Debug, Serialize)]
        struct Params {
            field: StructVariant,
        }
        // top level struct variant is supported
        {
            let params = StructVariant::A {
                username: String::from("boxdot"),
            };
            let url_params = to_string(&params);
            assert!(url_params.is_ok());
            assert_eq!(url_params.unwrap(), "username=boxdot");
        }
        // nested struct variant is not supported
        {
            let params = Params {
                field: StructVariant::A {
                    username: String::from("boxdot"),
                },
            };
            let url_params = to_string(&params);
            assert!(url_params.is_err());
        }
    }

    #[test]
    fn test_urlencoded() {
        #[derive(Debug, Serialize)]
        struct Params {
            field: String,
        }
        let params = Params {
            field: String::from("{some=weird&param}"),
        };
        let url_params = to_string(&params);
        assert!(url_params.is_ok());
        assert_eq!(url_params.unwrap(), "field=%7Bsome%3Dweird%26param%7D");
    }

    #[test]
    fn test_flattened_struct() {
        #[derive(Serialize, Debug)]
        pub struct Complex {
            real: f64,
            imag: f64,
        }

        #[derive(Serialize, Debug)]
        pub struct Params {
            x: u64,
            #[serde(flatten)]
            z: Option<Complex>,
        }

        let params = Params {
            x: 1,
            z: Some(Complex {
                real: 0.0,
                imag: 1.0,
            }),
        };
        let url_params = to_string(&params);
        assert_eq!(
            url_params.expect("failed serialization"),
            "x=1&real=0&imag=1"
        );
    }

    #[test]
    fn test_seq_of_struct() {
        #[derive(Serialize, Debug)]
        pub struct Complex {
            real: f64,
            imag: f64,
        }

        #[derive(Serialize, Debug)]
        #[serde(transparent)]
        pub struct Params {
            seq: Vec<Complex>,
        }

        let params = Params {
            seq: vec![
                Complex {
                    real: 0.0,
                    imag: 1.0,
                },
                Complex {
                    real: 1.0,
                    imag: 0.0,
                },
            ],
        };
        let url_params = to_string(&params);
        assert_eq!(
            url_params.expect("failed serialization"),
            "real=0&imag=1&real=1&imag=0"
        );
    }
}
