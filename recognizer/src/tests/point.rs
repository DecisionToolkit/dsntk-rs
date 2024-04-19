use crate::point::*;

#[test]
fn test_point_zero() {
  let p = Point::default();
  assert_eq!(p.x, 0);
  assert_eq!(p.y, 0);
}

#[test]
fn test_point_new() {
  let p = Point::new(1, 2);
  assert_eq!(p.x, 1);
  assert_eq!(p.y, 2);
  p.assert_receiver_is_total_eq();
}

#[test]
fn test_point_display() {
  assert_eq!("(10,20)", format!("{}", Point::new(10, 20)));
  assert_eq!("(0,0)", format!("{}", Point::new(0, 0)));
}

#[test]
fn test_point_debug() {
  assert_eq!("(10,20)", format!("{:?}", Point::new(10, 20)));
  assert_eq!("(0,0)", format!("{:?}", Point::new(0, 0)));
}

#[test]
fn test_point_compare() {
  let p1 = Point::new(1, 2);
  let p2 = Point::new(1, 2);
  let p3 = Point::new(2, 1);
  assert!(p1.eq(&p2));
  assert!(p1.ne(&p3));
}

#[test]
fn test_point_into_inner() {
  assert_eq!((21, 22), Point::new(21, 22).into_inner());
}

#[test]
fn test_point_clone() {
  let p1 = Point::new(1, 2);
  assert!(p1.eq(&p1.clone()));
}
