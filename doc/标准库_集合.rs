// 
// Collections
//      Sequences:
//          Vec: 尾端插入, 通用数组集合, Stack
//          VecDeque: 两端高效插入, Queue, Deque
//          LinkedList: 高效插入、拆分
//      Maps:
//          HashMap: key-value, cache
//          BTreeMap: 排序 Key-value
//      Sets: HashSet, BTreeSet
//      Misc:
//          BinaryHeap: 最大最小堆
//  迭代器方法:
//      iter: &T
//      iter_mut: &mut T
//      into_iter: T
//  std::vec::Vec:
//      init:
//          pub const fn new() -> Vec<T, Global>
//          pub fn with_capacity(capacity: usize) -> Vec<T, Global>
//      basic:
//          pub fn len(&self) -> usize
//          pub fn capacity(&self) -> usize
//      retrive:
//          pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
//          pub fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>
//      stack:
//          pub fn push(&mut self, value: T)
//          pub fn pop(&mut self) -> Option<T>
//      append:
//          pub fn append(&mut self, other: &mut Vec<T, A>)
//          pub fn insert(&mut self, index: usize, element: T)
//      delete:
//          pub fn remove(&mut self, index: usize) -> T
//          pub fn clear(&mut self)
//          pub fn truncate(&mut self, len: usize)
//          pub fn shrink_to_fit(&mut self)
//      covert:
//          pub fn into_boxed_slice(self) -> Box<[T], A>
//          pub fn as_slice(&self) -> &[T]
//          pub fn as_mut_slice(&mut self) -> &mut [T]
// std::collections::VecDeque:
//      like Vec
//      queue:
//          pub fn push_front(&mut self, value: T)
//          pub fn push_back(&mut self, value: T)
//          pub fn pop_front(&mut self) -> Option<T>
//          pub fn pop_back(&mut self) -> Option<T>
// std::collections::HashMap:
//      init:
//          pub fn new() -> HashMap<K, V, RandomState>
//          pub fn with_capacity(capacity: usize) -> HashMap<K, V, RandomState>
//      basic:
//          pub fn len(&self) -> usize
//          pub fn capacity(&self) -> usize
//      retrive:
//          pub fn keys(&self) -> Keys<'_, K, V>: &'a K
//          pub fn values(&self) -> Values<'_, K, V>: &'a V
//          pub fn values_mut(&mut self) -> ValuesMut<'_, K, V>: &'a mut V
//          pub fn entry(&mut self, key: K) -> Entry<'_, K, V>
//          pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
//          pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
//          pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
//          pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
//          pub fn iter(&self) -> Iter<'_, K, V>: (&'a K, &'a V)
//          pub fn iter_mut(&mut self) -> IterMut<'_, K, V>: (&'a K, &'a mut V)
//      delete:
//          pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
//          pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
//          pub fn clear(&mut self)
//          pub fn shrink_to_fit(&mut self)
//      append:
//          pub fn insert(&mut self, k: K, v: V) -> Option<V>
// std::collections::HashSet:
//      init:
//          pub fn new() -> HashSet<T, RandomState>
//          pub fn with_capacity(capacity: usize) -> HashSet<T, RandomState>
//      basic:
//          pub fn len(&self) -> usize
//          pub fn capacity(&self) -> usize
//      retrive:
//          pub fn iter(&self) -> Iter<'_, T>
//          pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<&T>
//      append:
//          pub fn insert(&mut self, value: T) -> bool
//      delete:
//          pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool
//          pub fn take<Q: ?Sized>(&mut self, value: &Q) -> Option<T>
//          pub fn clear(&mut self)
