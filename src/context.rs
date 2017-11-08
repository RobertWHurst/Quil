use std::fmt;
use std::collections::{hash_map, HashMap};

/// Creates a [`Context`](struct.Context.html) from a varying set of
/// keys and values.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate quil;
/// # fn main() {
/// # use quil::prelude::*;
/// let ctx: Context = context!{ key: "value" };
/// assert_eq!(ctx.get("key").unwrap(), "value");
/// }
/// ```
#[macro_export]
macro_rules! context {
  ($($key:ty: $value:expr),*) => ({
    use $crate::Context;

    let mut context = Context::new();
    $(
      context.set(stringify!($key), $value);
    )*
    context
  })
}

/// A logging context.
///
/// Contexts contain meta data for the association with logged messages.
#[derive(Debug, Clone)]
pub struct Context(HashMap<String, String>);

impl Context {
  /// Creates a new context for use with a logger.
  pub fn new() -> Self {
    Self { 0: HashMap::new() }
  }

  /// Sets a value within the context by a given key.
  ///
  /// # Arguments
  ///
  /// * `key` - A key for the value.
  /// * `value` - The value to add to the context.
  ///
  /// # Examples
  ///
  /// Setting a value:
  ///
  /// ```rust
  /// # use quil::prelude::*;
  /// # let mut ctx = Context::new();
  /// ctx.set("key", "value");
  /// ```
  ///
  /// Clearing a value:
  ///
  /// ```rust
  /// # use quil::prelude::*;
  /// # let mut ctx = Context::new();
  /// ctx.set("key", "");
  /// ```
  pub fn set<S>(&mut self, key: S, value: S)
  where
    S: Into<String>,
  {
    let key = key.into();
    let value = value.into();
    if value == "" {
      self.0.remove(&key);
    } else {
      self.0.insert(key, value);
    }
  }

  /// Gets a value by key.
  ///
  /// If the key does not exist then None will be returned.
  ///
  /// # Arguments
  ///
  /// * `key` - A key for the value.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use quil::prelude::*;
  /// # let mut ctx = Context::new();
  /// ctx.set("key", "value");
  /// assert_eq!(ctx.get("key").unwrap(), "value");
  /// ```
  pub fn get(&self, key: &str) -> Option<&String> {
    self.0.get(key)
  }

  /// Create a new merged context from the context `merge` is called upon and
  /// a given context. The given context's values take precedence.
  ///
  /// # Arguments
  ///
  /// * `context` - A context that will be merged.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # #[macro_use] extern crate quil;
  /// # fn main() {
  /// # use quil::prelude::*;
  /// let context_a = context!{ a: "1", b: "2" };
  /// let context_b = context!{ b: "3", c: "4" };
  ///
  /// let merged_context = context_a.merge(context_b);
  ///
  /// assert_eq!(merged_context.get("a").unwrap(), "1");
  /// assert_eq!(merged_context.get("b").unwrap(), "3");
  /// assert_eq!(merged_context.get("c").unwrap(), "4");
  /// # }
  /// ```
  pub fn merge(&self, context: Self) -> Self {
    let mut merged_context = Self { 0: self.0.clone() };
    for (key, val) in context.0.into_iter() {
      merged_context.set(key, val);
    }
    merged_context
  }

  /// Returns an iterator to iterate through the data contained within
  /// the context.
  pub fn iter(&self) -> hash_map::Iter<String, String> {
    self.0.iter()
  }

  /// Consumes the context returning an iterator to iterate through the
  /// data once contained within the context.
  pub fn into_iter(self) -> hash_map::IntoIter<String, String> {
    self.0.into_iter()
  }
}

impl fmt::Display for Context {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let messages: Vec<_> = self
      .0
      .iter()
      .map(|(key, val)| format!("{}: \"{}\"", key, val))
      .collect();
    write!(f, "{{ {} }}", messages.join(", "))
  }
}
