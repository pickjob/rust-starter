// 
// Iterator:
//      iter: &T
//      iter_mut: &mut T
//      into_iter: T 
//  traits:
//      pub trait Iterator {
//          type Item;
//          fn next(&mut self) -> Option<Self::Item>;
//      }
//          mindle:
//              pub fn filter<P>(self, predicate: P) -> Filter<Self, P>
//              pub fn map<B, F>(self, f: F) -> Map<Self, F>
//              pub fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
//              pub fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
//              pub fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>
//              pub fn enumerate(self) -> Enumerate<Self>: (idx, val)
//              pub fn skip(self, n: usize) -> Skip<Self>
//              pub fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
//              pub fn take(self, n: usize) -> Take<Self>
//              pub fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
//          terminal:
//              pub fn for_each<F>(self, f: F)
//              pub fn count(self) -> usize
//              pub fn sum<S>(self) -> S
//              pub fn last(self) -> Option<Self::Item>
//              pub fn nth(&mut self, n: usize) -> Option<Self::Item>
//              pub fn collect<B>(self) -> B
//              pub fn partition<B, F>(self, f: F) -> (B, B)
//              pub fn fold<B, F>(self, init: B, f: F) -> B
//              pub fn reduce<F>(self, f: F) -> Option<Self::Item>
//              pub fn all<F>(&mut self, f: F) -> bool
//              pub fn any<F>(&mut self, f: F) -> bool
//              pub fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
//              pub fn position<P>(&mut self, predicate: P) -> Option<usize>
//              pub fn max(self) -> Option<Self::Item>
//              pub fn min(self) -> Option<Self::Item>
//      pub trait IntoIterator {
//          type Item;
//          type IntoIter: Iterator;
//          pub fn into_iter(self) -> Self::IntoIter;
//      }
