use crate::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Mutex;
//                                 name    data
type Table<T> = Lazy<Mutex<HashMap<String, T>>>;

pub static V_MAP: Table<(String, Range<usize>, i64)> = Lazy::new(|| Mutex::new(HashMap::new()));

pub static LABEL_MAP: Table<(String, Range<usize>, usize)> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static MACRO_MAP: Table<(String, MacroContent)> = Lazy::new(|| Mutex::new(HashMap::new()));
