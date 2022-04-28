// 
// rand: 随机数生成工具
//      prelue: rand::prelude::*;
//      trait:
//          pub trait RngCore {
//              fn next_u32(&mut self) -> u32;
//              fn next_u64(&mut self) -> u64;
//              fn fill_bytes(&mut self, dest: &mut [u8]);
//              fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error>;
//          }
//          pub trait Rng: RngCore {
//              fn gen<T>(&mut self) -> T where Standard: Distribution<T>, { ... }
//              fn gen_range<T, R>(&mut self, range: R) -> T where T: SampleUniform, R: SampleRange<T>, { ... }
//              fn sample<T, D: Distribution<T>>(&mut self, distr: D) -> T { ... }
//              fn sample_iter<T, D>(self, distr: D) -> DistIter<D, Self, T> where D: Distribution<T>, Self: Sized, { ... }
//              fn fill<T: Fill + ?Sized>(&mut self, dest: &mut T) { ... }
//              fn try_fill<T: Fill + ?Sized>(&mut self, dest: &mut T) -> Result<(), Error> { ... }
//              fn gen_bool(&mut self, p: f64) -> bool { ... }
//              fn gen_ratio(&mut self, numerator: u32, denominator: u32) -> bool { ... }
//          }
//          impl<R: RngCore + ?Sized> Rng for R
//          pub trait Fill {
//              fn try_fill<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Result<(), Error>;
//          }
//      function:
//          rand::thread_rng() -> ThreadRng
