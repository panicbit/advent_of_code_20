use std::{fmt::Debug, str::FromStr};
use regex::Captures;

#[doc(hidden)] pub use lazy_static::lazy_static;
#[doc(hidden)] pub use regex::Regex;

#[macro_export]
macro_rules! re {
    ($re:expr, $str:expr) => {{
        let re = re!($re);
        re.captures($str).unwrap()
    }};
    ($re:expr) => {{
        crate::lazy_static! {
            static ref RE: crate::Regex = crate::Regex::new($re).unwrap();
        }

        &*RE
    }};
}

pub trait CapturesExt<'a> {
    fn str(self, name: &str) -> &'a str;
    fn parse<T: FromStr>(self, name: &str) -> T
    where
        T::Err: Debug;
}

impl<'a> CapturesExt<'a> for &'_ Captures<'a> {
    fn str(self, name: &str) -> &'a str {
        self.name(name).unwrap().as_str()
    }

    fn parse<T: FromStr>(self, name: &str) -> T
    where
        T::Err: Debug
    {
        self.str(name).parse().unwrap()
    }
}
