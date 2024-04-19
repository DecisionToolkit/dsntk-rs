use crate::rect::*;

#[test]
fn test_rect_zero() {
  let r = Rect::default();
  assert_eq!(r.left, 0);
  assert_eq!(r.top, 0);
  assert_eq!(r.right, 0);
  assert_eq!(r.bottom, 0);
}

#[test]
fn test_rect_new() {
  let r = Rect::new(10, 0, 50, 100);
  assert_eq!(r.left, 10);
  assert_eq!(r.top, 0);
  assert_eq!(r.right, 50);
  assert_eq!(r.bottom, 100);
  r.assert_receiver_is_total_eq();
}

#[test]
fn test_rect_offset_top() {
  let r = Rect::new(10, 0, 50, 100);
  let s = r.offset_top(2);
  assert_eq!(s.left, 10);
  assert_eq!(s.top, 2);
  assert_eq!(s.right, 50);
  assert_eq!(s.bottom, 100);
}

#[test]
fn test_rect_into_inner() {
  let r = Rect::new(10, 11, 12, 13);
  let (left, top, right, bottom) = r.into_inner();
  assert_eq!(left, 10);
  assert_eq!(top, 11);
  assert_eq!(right, 12);
  assert_eq!(bottom, 13);
}

#[test]
fn test_rect_contains() {
  let r = Rect::new(10, 10, 20, 20);
  let s1 = Rect::new(10, 10, 20, 20);
  assert!(r.contains(&s1));
  let s2 = Rect::new(15, 15, 20, 20);
  assert!(r.contains(&s2));
  let s3 = Rect::new(10, 10, 15, 15);
  assert!(r.contains(&s3));
  let s4 = Rect::new(9, 9, 20, 20);
  assert!(!r.contains(&s4));
  let s5 = Rect::new(10, 10, 21, 21);
  assert!(!r.contains(&s5));
}

#[test]
fn test_rect_width_height() {
  assert_eq!(Rect::default().width(), 0);
  assert_eq!(Rect::new(0, 0, 0, 0).width(), 0);
  assert_eq!(Rect::new(0, 0, 1, 0).width(), 1);
  assert_eq!(Rect::new(0, 0, 10, 0).width(), 10);
  assert_eq!(Rect::default().height(), 0);
  assert_eq!(Rect::new(0, 0, 0, 0).height(), 0);
  assert_eq!(Rect::new(0, 0, 0, 1).height(), 1);
  assert_eq!(Rect::new(0, 0, 0, 10).height(), 10);
  let r = Rect::new(0, 0, 11, 11);
  assert_eq!(r.width(), 11);
  assert_eq!(r.height(), 11);
}

#[test]
fn test_rect_display() {
  assert_eq!("(10,11;12,13)", format!("{}", Rect::new(10, 11, 12, 13)));
}

#[test]
fn test_rect_debug() {
  assert_eq!("(10,11;12,13)", format!("{:?}", Rect::new(10, 11, 12, 13)));
}

#[test]
fn test_rect_compare() {
  let r1 = Rect::new(10, 10, 20, 20);
  let r2 = Rect::new(10, 10, 20, 20);
  let r3 = Rect::new(15, 15, 19, 19);
  assert!(r1.eq(&r2));
  assert!(r1.ne(&r3));
}

#[test]
fn test_rect_clone() {
  let r1 = Rect::new(10, 10, 20, 20);
  assert!(r1.eq(&r1.clone()));
}
