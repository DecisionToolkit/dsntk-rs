use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_bool(false, &scope!(), r#"after(10,1)"#, true);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), r#"after(date("2022-01-01"),date("2021-12-31"))"#, true);
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), r#"after(time("11:38:23"),time("11:38:22"))"#, true);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"after(date and time("2021-01-01T11:38:23"),date and time("2021-01-01T11:38:22"))"#, true);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"after(duration("P1DT11H38M23S"),duration("P1DT11H38M22S"))"#, true);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"after(duration("P1Y1M"),duration("P1Y"))"#, true);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"after(1,10)"#, false);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"after(date("2021-12-31"),date("2022-01-01"))"#, false);
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), r#"after(time("11:38:22"),time("11:38:23"))"#, false);
}

#[test]
fn _0010() {
  te_bool(false, &scope!(), r#"after(date and time("2021-01-01T11:38:22"),date and time("2021-01-01T11:38:23"))"#, false);
}

#[test]
fn _0011() {
  te_bool(false, &scope!(), r#"after(duration("P1DT11H38M22S"),duration("P1DT11H38M23S"))"#, false);
}

#[test]
fn _0012() {
  te_bool(false, &scope!(), r#"after(duration("P1Y"),duration("P1Y1M"))"#, false);
}

#[test]
fn _0013() {
  te_bool(false, &scope!(), r#"after(point1:10,point2:1)"#, true);
}

#[test]
fn _0014() {
  te_bool(false, &scope!(), r#"after(point1:date("2022-01-01"),point2:date("2021-12-31"))"#, true);
}

#[test]
fn _0015() {
  te_bool(false, &scope!(), r#"after(point1:time("11:38:23"),point2:time("11:38:22"))"#, true);
}

#[test]
fn _0016() {
  te_bool(false, &scope!(), r#"after(point1:date and time("2021-01-01T11:38:23"),point2:date and time("2021-01-01T11:38:22"))"#, true);
}

#[test]
fn _0017() {
  te_bool(false, &scope!(), r#"after(point1:duration("P1DT11H38M23S"),point2:duration("P1DT11H38M22S"))"#, true);
}

#[test]
fn _0018() {
  te_bool(false, &scope!(), r#"after(point1:duration("P1Y1M"),point2:duration("P1Y"))"#, true);
}

#[test]
fn _0019() {
  te_bool(false, &scope!(), r#"after(point1:1,point2:10)"#, false);
}

#[test]
fn _0020() {
  te_bool(false, &scope!(), r#"after(point1:date("2021-12-31"),point2:date("2022-01-01"))"#, false);
}

#[test]
fn _0021() {
  te_bool(false, &scope!(), r#"after(point1:time("11:38:22"),point2:time("11:38:23"))"#, false);
}

#[test]
fn _0022() {
  te_bool(false, &scope!(), r#"after(point1:date and time("2021-01-01T11:38:22"),point2:date and time("2021-01-01T11:38:23"))"#, false);
}

#[test]
fn _0023() {
  te_bool(false, &scope!(), r#"after(point1:duration("P1Y"),point2:duration("P1Y1M"))"#, false);
}

#[test]
fn _0024() {
  te_bool(false, &scope!(), r#"after(point1:duration("P1Y"),point2:duration("P1Y1M"))"#, false);
}

#[test]
fn _0025() {
  te_bool(false, &scope!(), r#"after(point2:1,point1:10)"#, true);
}

#[test]
fn _0026() {
  te_bool(false, &scope!(), r#"after(point2:date("2021-12-31"),point1:date("2022-01-01"))"#, true);
}

#[test]
fn _0027() {
  te_bool(false, &scope!(), r#"after(point2:time("11:38:22"),point1:time("11:38:23"))"#, true);
}

#[test]
fn _0028() {
  te_bool(false, &scope!(), r#"after(point2:date and time("2021-01-01T11:38:22"),point1:date and time("2021-01-01T11:38:23"))"#, true);
}

#[test]
fn _0029() {
  te_bool(false, &scope!(), r#"after(point2:duration("P1DT11H38M22S"),point1:duration("P1DT11H38M23S"))"#, true);
}

#[test]
fn _0030() {
  te_bool(false, &scope!(), r#"after(point2:duration("P1Y"),point1:duration("P1Y1M"))"#, true);
}

#[test]
fn _0031() {
  te_bool(false, &scope!(), r#"after(point2: 10, point1: 1)"#, false);
}

#[test]
fn _0032() {
  te_bool(false, &scope!(), r#"after(point2:date("2022-01-01"),point1:date("2021-12-31"))"#, false);
}

#[test]
fn _0033() {
  te_bool(false, &scope!(), r#"after(point2:time("11:38:23"),point1:time("11:38:22"))"#, false);
}

#[test]
fn _0034() {
  te_bool(false, &scope!(), r#"after(point2:date and time("2021-01-01T11:38:23"),point1:date and time("2021-01-01T11:38:22"))"#, false);
}

#[test]
fn _0035() {
  te_bool(false, &scope!(), r#"after(point2:duration("P1DT11H38M23S"),point1:duration("P1DT11H38M22S"))"#, false);
}

#[test]
fn _0036() {
  te_bool(false, &scope!(), r#"after(point2:duration("P1Y1M"),point1:duration("P1Y"))"#, false);
}

#[test]
fn _0037() {
  te_bool(false, &scope!(), r#"after(11,[1..10])"#, true);
}

#[test]
fn _0038() {
  te_bool(false, &scope!(), r#"after(date("2021-06-01"),[date("2021-01-01")..date("2021-05-30")])"#, true);
}

#[test]
fn _0039() {
  te_bool(false, &scope!(), r#"after(time("10:11:12"),[time("00:00:00")..time("10:11:11")])"#, true);
}

#[test]
fn _0040() {
  te_bool(false, &scope!(), r#"after(date and time("2021-01-01T10:11:12"),[date and time("2021-01-01T00:00:00")..date and time("2021-01-01T10:11:11")])"#, true);
}

#[test]
fn _0041() {
  te_bool(false, &scope!(), r#"after(duration("P1DT10H11M12S"),[duration("P1DT")..duration("P1DT10H11M11S")])"#, true);
}

#[test]
fn _0042() {
  te_bool(false, &scope!(), r#"after(duration("P2Y1M"),[duration("P1Y")..duration("P2Y")])"#, true);
}

#[test]
fn _0043() {
  te_bool(false, &scope!(), r#"after([1..10],11)"#, false);
}

#[test]
fn _0044() {
  te_bool(false, &scope!(), r#"after([date("2021-01-01")..date("2021-05-30")],date("2021-06-01"))"#, false);
}

#[test]
fn _0045() {
  te_bool(false, &scope!(), r#"after([time("00:00:00")..time("10:11:11")],time("10:11:12"))"#, false);
}

#[test]
fn _0046() {
  te_bool(false, &scope!(), r#"after([date and time("2021-01-01T00:00:00")..date and time("2021-01-01T10:11:11")],date and time("2021-01-01T10:11:12"))"#, false);
}

#[test]
fn _0047() {
  te_bool(false, &scope!(), r#"after([duration("P1DT")..duration("P1DT10H11M11S")],duration("P1DT10H11M12S"))"#, false);
}

#[test]
fn _0048() {
  te_bool(false, &scope!(), r#"after([duration("P1Y")..duration("P2Y")],duration("P1Y1M"))"#, false);
}

#[test]
fn _0049() {
  te_bool(false, &scope!(), r#"after(point:11,range:[1..10])"#, true);
}

#[test]
fn _0050() {
  te_bool(false, &scope!(), r#"after(point:date("2021-06-01"),range:[date("2021-01-01")..date("2021-05-30")])"#, true);
}

#[test]
fn _0051() {
  te_bool(false, &scope!(), r#"after(point:time("10:11:12"),range:[time("00:00:00")..time("10:11:11")])"#, true);
}

#[test]
fn _0052() {
  te_bool(false, &scope!(), r#"after(point:date and time("2021-01-01T10:11:12"),range:[date and time("2021-01-01T00:00:00")..date and time("2021-01-01T10:11:11")])"#, true);
}

#[test]
fn _0053() {
  te_bool(false, &scope!(), r#"after(point:duration("P1DT10H11M12S"),range:[duration("P1DT")..duration("P1DT10H11M11S")])"#, true);
}

#[test]
fn _0054() {
  te_bool(false, &scope!(), r#"after(point:duration("P2Y1M"),range:[duration("P1Y")..duration("P2Y")])"#, true);
}

#[test]
fn _0055() {
  te_bool(false, &scope!(), r#"after(range:[1..10],point:11)"#, false);
}

#[test]
fn _0056() {
  te_bool(false, &scope!(), r#"after(range:[date("2021-01-01")..date("2021-05-30")],point:date("2021-06-01"))"#, false);
}

#[test]
fn _0057() {
  te_bool(false, &scope!(), r#"after(range:[time("00:00:00")..time("10:11:11")],point:time("10:11:12"))"#, false);
}

#[test]
fn _0058() {
  te_bool(false, &scope!(), r#"after(range:[date and time("2021-01-01T00:00:00")..date and time("2021-01-01T10:11:11")],point:date and time("2021-01-01T10:11:12"))"#, false);
}

#[test]
fn _0059() {
  te_bool(false, &scope!(), r#"after(range:[duration("P1DT")..duration("P1DT10H11M11S")],point:duration("P1DT10H11M12S"))"#, false);
}

#[test]
fn _0060() {
  te_bool(false, &scope!(), r#"after(range:[duration("P1Y")..duration("P2Y")],point:duration("P1Y1M"))"#, false);
}

#[test]
fn _0061() {
  te_bool(false, &scope!(), r#"after(range1:[11..20],range2:[1..10])"#, true);
}

#[test]
fn _0062() {
  te_bool(false, &scope!(), r#"after(range1:[date("2021-01-06")..date("2021-01-23")],range2:[date("2020-11-18")..date("2021-01-05")])"#, true);
}

#[test]
fn _0063() {
  te_bool(false, &scope!(), r#"after(range1:[time("12:00:00")..time("23:59:59")],range2:[time("00:00:00")..time("11:59:59")])"#, true);
}

#[test]
fn _0064() {
  te_bool(false, &scope!(), r#"after(range1:[date and time("2021-01-01T12:00:00")..date and time("2021-01-01T23:59:59")],range2:[date and time("2021-01-01T00:00:00")..date and time("2021-01-01T11:59:59")])"#, true);
}

#[test]
fn _0065() {
  te_bool(false, &scope!(), r#"after(range1:[duration("P1DT12H")..duration("P1DT23H59M59S")],range2:[duration("P1DT")..duration("P1DT11H59M59S")])"#, true);
}

#[test]
fn _0066() {
  te_bool(false, &scope!(), r#"after(range1:[duration("P2Y")..duration("P2Y3M")],range2:[duration("P1Y")..duration("P1Y11M")])"#, true);
}

#[test]
fn _0067() {
  te_bool(false, &scope!(), r#"after(range1:[1..10],range2:[11..20])"#, false);
}

#[test]
fn _0068() {
  te_bool(false, &scope!(), r#"after(range1:[date("2020-11-18")..date("2021-01-05")],range2:[date("2021-01-06")..date("2021-01-23")])"#, false);
}

#[test]
fn _0069() {
  te_bool(false, &scope!(), r#"after(range1:[time("00:00:00")..time("11:59:59")],range2:[time("12:00:00")..time("23:59:59")])"#, false);
}

#[test]
fn _0070() {
  te_bool(false, &scope!(), r#"after(range1:[date and time("2021-01-01T00:00:00")..date and time("2021-01-01T11:59:59")],range2:[date and time("2021-01-01T12:00:00")..date and time("2021-01-01T23:59:59")])"#, false);
}

#[test]
fn _0071() {
  te_bool(false, &scope!(), r#"after(range1:[duration("P1DT")..duration("P1DT11H59M59S")],range2:[duration("P1DT12H")..duration("P1DT23H59M59S")])"#, false);
}

#[test]
fn _0072() {
  te_bool(false, &scope!(), r#"after(range1:[duration("P1Y6M")..duration("P2Y")],range2:[duration("P1Y")..duration("P1Y7M")])"#, false);
}

#[test]
fn _0073() {
  te_bool(false, &scope!(), r#"after(range2:[1..10],range1:[11..20])"#, true);
}

#[test]
fn _0074() {
  te_bool(false, &scope!(), r#"after(range2:[date("2020-11-18")..date("2021-01-05")],range1:[date("2021-01-06")..date("2021-01-23")])"#, true);
}

#[test]
fn _0075() {
  te_bool(false, &scope!(), r#"after(range2:[time("00:00:00")..time("11:59:59")],range1:[time("12:00:00")..time("23:59:59")])"#, true);
}

#[test]
fn _0076() {
  te_bool(false, &scope!(), r#"after(range2:[date and time("2021-01-01T00:00:00")..date and time("2021-01-01T11:59:59")],range1:[date and time("2021-01-01T12:00:00")..date and time("2021-01-01T23:59:59")])"#, true);
}

#[test]
fn _0077() {
  te_bool(false, &scope!(), r#"after(range2:[duration("P1DT")..duration("P1DT11H59M59S")],range1:[duration("P1DT12H")..duration("P1DT23H59M59S")])"#, true);
}

#[test]
fn _0078() {
  te_bool(false, &scope!(), r#"after(range2:[duration("P1Y")..duration("P2Y")],range1:[duration("P2Y1M")..duration("P3Y")])"#, true);
}

#[test]
fn _0079() {
  te_bool(false, &scope!(), r#"after(range2:[11..20],range1:[1..10])"#, false);
}

#[test]
fn _0080() {
  te_bool(false, &scope!(), r#"after(range2:[date("2021-01-06")..date("2021-01-23")],range1:[date("2020-11-18")..date("2021-01-05")])"#, false);
}

#[test]
fn _0081() {
  te_bool(false, &scope!(), r#"after(range2:[time("12:00:00")..time("23:59:59")],range1:[time("00:00:00")..time("11:59:59")])"#, false);
}

#[test]
fn _0082() {
  te_bool(false, &scope!(), r#"after(range2:[date and time("2021-01-01T12:00:00")..date and time("2021-01-01T23:59:59")],range1:[date and time("2021-01-01T00:00:00")..date and time("2021-01-01T11:59:59")])"#, false);
}

#[test]
fn _0083() {
  te_bool(false, &scope!(), r#"after(range2:[duration("P1DT12H")..duration("P1DT23H59M59S")],range1:[duration("P1DT")..duration("P1DT11H59M59S")])"#, false);
}

#[test]
fn _0084() {
  te_bool(false, &scope!(), r#"after(range2:[duration("P2Y1M")..duration("P3Y")],range1:[duration("P1Y")..duration("P2Y")])"#, false);
}

#[test]
fn _0085() {
  te_null(false, &scope!(), r#"after(p1:10,point2:1)"#, r#"[named::after] invalid named parameters"#);
}

#[test]
fn _0086() {
  te_null(false, &scope!(), r#"after()"#, r#"expected 2 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0087() {
  te_null(false, &scope!(), r#"after(1,2,3)"#, r#"expected 2 parameters, actual number of parameters is 3"#);
}

#[test]
fn _0088() {
  te_null(false, &scope!(), r#"after(null,null)"#, r#"[core::after] invalid argument type, expected scalar or range of scalars, actual type is Null"#);
}

#[test]
fn _0089() {
  te_null(false, &scope!(), r#"after(1,null)"#, r#"[core::after] invalid argument type, expected scalar or range of scalars, actual type is number"#);
}

#[test]
fn _0090() {
  te_null(false, &scope!(), r#"after(null,1)"#, r#"[core::after] invalid argument type, expected scalar or range of scalars, actual type is Null"#);
}
