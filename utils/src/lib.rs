use std::{
    collections::BTreeMap,
    fmt::Debug,
    str::FromStr,
    sync::Mutex,
};
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

pub trait StrExt {
    fn u8(&self) -> u8;
    fn i32(&self) -> i32;
    fn usize(&self) -> usize;
    fn is_match(&self, regex: &str) -> bool;
}

impl<S: AsRef<str>> StrExt for S {
    fn u8(&self) -> u8 {
        self.as_ref().parse().unwrap()
    }

    fn i32(&self) -> i32 {
        self.as_ref().parse().unwrap()
    }

    fn usize(&self) -> usize {
        self.as_ref().parse().unwrap()
    }

    fn is_match(&self, regex: &str) -> bool {
        with_cached_regex(regex, |regex| {
            regex.is_match(self.as_ref())
        })
    }
}

impl StrExt for str {
    fn u8(&self) -> u8 {
        self.parse().unwrap()
    }

    fn i32(&self) -> i32 {
        self.parse().unwrap()
    }

    fn usize(&self) -> usize {
        self.parse().unwrap()
    }

    fn is_match(&self, regex: &str) -> bool {
        with_cached_regex(regex, |regex| {
            regex.is_match(self.as_ref())
        })
    }
}

// TODO: optimize for concurrent access
fn with_cached_regex<F, R>(regex: &str, f: F) -> R
where
    F: FnOnce(&Regex) -> R,
{
    let mut regex_cache = REGEX_CACHE.lock().unwrap();

    let regex = match regex_cache.get(regex) {
        Some(regex) => regex,
        None => {
            let compiled_regex = Regex::new(regex).unwrap();
            regex_cache.insert(regex.into(), compiled_regex);
            regex_cache.get(regex).unwrap()
        }
    };

    f(regex)
}

lazy_static! {
    static ref REGEX_CACHE: Mutex<BTreeMap<String, Regex>> = Mutex::default();
}
