use super::super::Context;
use super::super::Level;
use super::Target;

/// Creates a [`TargetSet`](struct.TargetSet.html) from a varying set of
/// targets. [`TargetSet`](struct.TargetSet.html) allows the logger to log
/// to more than a single target at once.
#[macro_export]
macro_rules! targets {
    ($($target:expr),+) => ({
        use $crate::targets::TargetSet;
        let mut target_set = TargetSet::new();
        $(
            target_set.add($target);
        )+
        target_set
    });
}

/// A target that writes logged messages to a collection of targets contained
/// with in.
///
/// A `TargetSet` contains a collection of struct instances that implement
/// [`Target`](trait.Target.html). `TargetSet` also implements
/// [`Target`](trait.Target.html) so it can be passed to
/// [`Logger`](struct.Logger.html). A message logged to the `TargetSet`
/// will be logged to each of the targets within it.
pub struct TargetSet<'a> {
  targets: Vec<Box<Target + 'a>>,
}

impl<'a> TargetSet<'a> {
  /// Creates a new empty `TargetSet`.
  pub fn new() -> Self {
    Self {
      targets: Vec::new(),
    }
  }

  /// Add a target to the `TargetSet`.
  ///
  /// # Arguments
  ///
  /// * `target` - A type implementing [`target`](trait.Target.html).
  ///
  pub fn add<T>(&mut self, target: T)
  where
    T: Target + 'a,
  {
    self.targets.push(Box::new(target));
  }
}

impl<'a> Target for TargetSet<'a> {
  fn log(&mut self, level: Level, message: &str, context: &Context) {
    for target in self.targets.iter_mut() {
      target.log(level, message, context);
    }
  }
}
