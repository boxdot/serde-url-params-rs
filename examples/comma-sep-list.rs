//! Example how to serialize a list of parameter values as a comma-separated list (non-exploded)
use core::fmt;

use itertools::Itertools;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
struct AuthorizationParameters<'a> {
    scope: CommaSeparated<&'a str>,
}

struct CommaSeparated<T: fmt::Display>(Vec<T>);

impl<T: fmt::Display> Serialize for CommaSeparated<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let list = self.0.iter().format(",");
        serializer.collect_str(&format_args!("{}", list))
    }
}

fn main() {
    let params = AuthorizationParameters {
        scope: CommaSeparated(vec!["openid", "profile"]),
    };
    let expected = "scope=openid%2Cprofile";
    assert_eq!(serde_url_params::to_string(&params).unwrap(), expected);
}
