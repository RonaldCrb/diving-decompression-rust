#[cfg(test)]
use super::Dive;

#[test]
fn calculate_1() {
  let d = Dive::new(10, 222);
  let ndl = d.no_decompression_limit();
  let gl = d.group_letter();
  assert_eq!(ndl, 9999);
  assert_eq!(gl, String::from("D"));
}

#[test]
fn calculate_2() {
  let d = Dive::new(15, 222);
  let ndl = d.no_decompression_limit();
  let gl = d.group_letter();
  assert_eq!(ndl, 9999);
  assert_eq!(gl, String::from("G"));
}

#[test]
fn calculate_3() {
  let d = Dive::new(20, 461);
  let ndl = d.no_decompression_limit();
  let gl = d.group_letter();
  assert_eq!(ndl, 9999);
  assert_eq!(gl, String::from("K"));
}

#[test]
fn calculate_4() {
  let d = Dive::new(23, 461);
  let ndl = d.no_decompression_limit();
  let gl = d.group_letter();
  assert_eq!(ndl, 1102);
  assert_eq!(gl, String::from("N"));
}

#[test]
fn calculate_5() {
  let d = Dive::new(26, 461);
  let ndl = d.no_decompression_limit();
  let gl = d.group_letter();
  assert_eq!(ndl, 371);
  assert_eq!(gl, String::from("this dive is out of the time range for no-decompression air dives"));
}

#[test]
fn calculate_6() {
  let d = Dive::new(35, 42);
  let ndl = d.no_decompression_limit();
  let gl = d.group_letter();
  assert_eq!(ndl, 232);
  assert_eq!(gl, String::from("D"));
}

#[test]
fn calculate_7() {
  let d = Dive::new(39, 120);
  let ndl = d.no_decompression_limit();
  let gl = d.group_letter();
  assert_eq!(ndl, 163);
  assert_eq!(gl, String::from("L"));
}

#[test]
fn calculate_8() {
  let d = Dive::new(44, 120);
  let ndl = d.no_decompression_limit();
  let gl = d.group_letter();
  assert_eq!(ndl, 125);
  assert_eq!(gl, String::from("N"));
}

#[test]
fn calculate_9() {
  let d = Dive::new(46, 60);
  let ndl = d.no_decompression_limit();
  let gl = d.group_letter();
  assert_eq!(ndl, 92);
  assert_eq!(gl, String::from("I"));
}

#[test]
fn calculate_10() {
  let d = Dive::new(55, 74);
  let ndl = d.no_decompression_limit();
  let gl = d.group_letter();
  assert_eq!(ndl, 74);
  assert_eq!(gl, String::from("L"));
}