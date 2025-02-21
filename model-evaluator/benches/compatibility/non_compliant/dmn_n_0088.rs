use super::*;

#[bench]
fn _0001(_b: &mut Bencher) {
  // Measuring this time makes no sense, until parsing times are the subject.
  // iter!(b, dsntk_model::parse(DMN_N_0088).err().unwrap().to_string());
}
