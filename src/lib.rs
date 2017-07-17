extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_derive;

pub use self::error::{Error, Result};
pub use self::ser::{Serializer, to_string, to_vec, to_writer};

pub mod error;
pub mod ser;

#[cfg(test)]
mod tests {
    use super::to_string;

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
    fn test_tuple() {
        #[derive(Debug, Serialize)]
        struct Params {
            field: (usize, &'static str, f32),
        }
        let params = Params { field: (42, "hello", 3.14) };
        let url_params = to_string(&params);
        assert!(url_params.is_ok());
        assert_eq!(url_params.unwrap(), "field=42&field=hello&field=3.14");
    }

    #[test]
    fn test_tuple_struct() {
        #[derive(Debug, Serialize)]
        struct TupleStruct(usize, &'static str, f32);
        #[derive(Debug, Serialize)]
        struct Params {
            field: TupleStruct,
        }
        let params = Params { field: TupleStruct(42, "hello", 3.14) };
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
            let params = A { username: String::from("boxdot") };
            let url_params = to_string(&params);
            assert!(url_params.is_ok());
            assert_eq!(url_params.unwrap(), "username=boxdot");
        }
        // nested struct is not supported
        {
            let params = Params { field: A { username: String::from("boxdot") } };
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
            let params = StructVariant::A { username: String::from("boxdot") };
            let url_params = to_string(&params);
            assert!(url_params.is_ok());
            assert_eq!(url_params.unwrap(), "username=boxdot");
        }
        // nested struct variant is not supported
        {
            let params = Params { field: StructVariant::A { username: String::from("boxdot") } };
            let url_params = to_string(&params);
            assert!(url_params.is_err());
        }
    }
}
