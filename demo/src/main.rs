mod lazy;
mod other;
mod info;

use std::ops::Add;
use crate::lazy::{AAAAAAAAA, HMM};
use crate::other::hmm;

fn main() {
  let z = &AAAAAAAAA;
  let x = &HMM.add(2);
  hmm();
}
