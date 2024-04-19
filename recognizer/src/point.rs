//! # Point

use std::fmt;
use std::fmt::{Debug, Display};

/// Point.
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Point {
  /// Left coordinate.
  pub x: usize,
  /// Top coordinate.
  pub y: usize,
}

impl Display for Point {
  /// Implements [Display] trait for [Point].
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}

impl Debug for Point {
  /// Implements [Debug] trait for [Point].
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Point {
  /// Creates a new point with specified coordinates.
  pub fn new(x: usize, y: usize) -> Self {
    Point { x, y }
  }
}

impl From<Point> for (usize, usize) {
  /// Converts this point into a tuple of coordinates.
  fn from(value: Point) -> Self {
    (value.x, value.y)
  }
}
