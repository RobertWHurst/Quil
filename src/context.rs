use std::collections::HashMap;
use std::collections::hash_map;

#[derive(Debug, Clone)]
pub struct Context {
    pub data: HashMap<String, String>,
}

impl Context {
    pub fn merge<LC>(&self, src_context: LC) -> Self
    where
        LC: Into<Context>,
    {
        let src_context = src_context.into();
        let mut merged_context = Self { data: self.data.clone() };
        for (key, val) in src_context.data.iter() {
            merged_context.data.insert(key.to_string(), val.to_string());
        }
        merged_context
    }

    pub fn iter(&self) -> hash_map::Iter<String, String> {
        self.data.iter()
    }
}

impl From<HashMap<String, String>> for Context {
    fn from(data: HashMap<String, String>) -> Self {
        Self { data }
    }
}

impl<KS, VS> From<Vec<(KS, VS)>> for Context
where
    KS: Into<String>,
    VS: Into<String>,
{
    fn from(vec: Vec<(KS, VS)>) -> Self {
        let mut data = HashMap::new();
        for set in vec {
            data.insert(set.0.into(), set.1.into());
        }
        Self { data }
    }
}
