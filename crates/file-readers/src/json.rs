use serde_json::{from_str, Value};

pub enum MatchResult<'a> {
    SingleExact(&'a Value),
    Single(&'a Value),
    Keys(Vec<String>),
    All(Vec<&'a Value>),
    None,
}

#[derive(serde::Deserialize)]
pub enum Data {
    Vec(Vec<Value>),
    Value(Value),
}

pub struct Navigator {
    data: Data,
    index: usize,
}

impl Navigator {
    pub fn new(json: &str) -> Self {
        let parsed: Value = from_str(json).expect("Invalid JSON");
        let data = match parsed {
            Value::Array(arr) => Data::Vec(arr),
            _ => Data::Value(parsed),
        };
        Self { data, index: 0 }
    }

    pub fn find_by_key_value_adv(
        &self,
        key: &str,
        query: Option<&str>,
        return_all: bool,
        exact: bool,
    ) -> MatchResult {
        let mut matches = Vec::new();
        match &self.data {
            Data::Vec(vec) => {
                for item in vec {
                    if let Some(val) = item.get(key) {
                        let val_str = match val {
                            Value::String(s) => s.to_lowercase(),
                            other => other.to_string().to_lowercase(),
                        };

                        if let Some(q) = query {
                            let q_lower = q.to_lowercase();
                            if exact {
                                if val_str == q_lower {
                                    return MatchResult::SingleExact(item);
                                }
                            } else if val_str.contains(&q_lower) {
                                matches.push(item);
                            }
                        } else {
                            matches.push(item);
                        }
                    }
                }
            }
            Data::Value(val) => {
                matches.push(val.as_object().unwrap().get(key).unwrap());
            }
        }

        match matches.len() {
            0 => MatchResult::None,
            1 => MatchResult::Single(matches[0]),
            _ if return_all => MatchResult::All(matches),
            _ => {
                let keys = matches
                    .iter()
                    .filter_map(|item| item.get(key)?.as_str().map(|s| s.to_string()))
                    .collect();
                MatchResult::Keys(keys)
            }
        }
    }

    pub fn current(&self) -> Option<&Value> {
        match &self.data {
            Data::Vec(vec) => vec.get(self.index),
            Data::Value(val) => Some(val),
        }
    }

    pub fn next(&mut self) -> Option<&Value> {
        match &self.data {
            Data::Vec(vec) => {
                if self.index + 1 < vec.len() {
                    self.index += 1;
                }
                self.current()
            }
            Data::Value(_) => None,
        }
    }

    pub fn prev(&mut self) -> Option<&Value> {
        if self.index > 0 {
            self.index -= 1;
        }
        self.current()
    }

    pub fn find_by_key_value(&self, key: &str, val: &str) -> Option<&Value> {
        match &self.data {
            Data::Vec(vec) => vec.iter().find(|item| {
                if let Some(v) = item.get(key) {
                    if let Some(s) = v.as_str() {
                        return s == val;
                    }
                }
                false
            }),
            Data::Value(_) => None,
        }
    }

    pub fn find_nested(&self, path: &[&str], target: &str) -> Option<&Value> {
        match &self.data {
            Data::Vec(vec) => {
                for item in vec {
                    if let Some(val) = item.get(path[0]) {
                        if path.len() == 1 {
                            if let Some(s) = val.as_str() {
                                if s == target {
                                    return Some(item);
                                }
                            }
                        } else {
                            let nested_val = val.clone();
                            let navigator = Navigator {
                                data: Data::Value(nested_val),
                                index: 0,
                            };
                            if let Some(_) = navigator.find_nested(&path[1..], target) {
                                return Some(item);
                            }
                        }
                    }
                }
                None
            }
            Data::Value(_) => None,
        }
    }
    pub fn jump_to(&mut self, idx: usize) -> Option<&Value> {
        match &self.data {
            Data::Vec(vec) => {
                if idx < vec.len() {
                    self.index = idx;
                    Some(&vec[idx])
                } else {
                    None
                }
            }
            Data::Value(_) => None,
        }
    }

    pub fn len(&self) -> usize {
        match &self.data {
            Data::Vec(vec) => vec.len(),
            Data::Value(_) => 1,
        }
    }
}
