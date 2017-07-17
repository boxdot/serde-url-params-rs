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

}
