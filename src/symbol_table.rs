use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
// Where it is                   name      file     loc                     val
type Ltable = Lazy<Mutex<HashMap<String, (String, std::ops::Range<usize>, usize)>>>;
type Vtable = Lazy<Mutex<HashMap<String, (String, std::ops::Range<usize>, i64)>>>;

pub static VARIABLE_MAP: Vtable = Lazy::new(|| Mutex::new(HashMap::new()));

pub static LABEL_MAP: Ltable = Lazy::new(|| Mutex::new(HashMap::new()));
