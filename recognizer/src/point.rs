//! # Point

use std::fmt;

/// Point with coordinates set to `(0,0)`.
pub const POINT_ZERO: Point = Point { x: 0, y: 0 };

/// Vector of points.
pub type Points = Vec<Point>;

/// Point.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point {
  /// Left coordinate.
  pub x: usize,
  /// Top coordinate.
  pub y: usize,
}

impl fmt::Display for Point {
  /// Implements [Display](fmt::Display) trait for [Point].
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl fmt::Debug for Point {
  /// Implements [Debug](fmt::Debug) trait for [Point].
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}

impl Point {
  /// Creates a new point with specified coordinates.
  pub fn new(x: usize, y: usize) -> Point {
    Point { x, y }
  }

  /// Converts this point's coordinates to tuple of integers.
  pub fn into_inner(self) -> (usize, usize) {
    (self.x, self.y)
  }
}
