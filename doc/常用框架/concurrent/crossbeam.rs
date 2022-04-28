/**
 *  crossbeam: 并发工具
 *      Atomics:
 *          AtomicCell
 *          AtomicConsume
 *      Data structures:
 *          deque:
 *              Injector: FIFO queue
 *                  pub fn new() -> Injector<T>
 *                  pub fn steal(&self) -> Steal<T>
 *                  pub fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>
 *                  pub fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T>
 *                  pub fn push(&self, task: T)
 *                  pub fn is_empty(&self) -> bool 是否存在
 *                  pub fn len(&self) -> usize
 *              Worker:
 *                  pub fn new_fifo() -> Worker<T>: FIFO queue
 *                  pub fn new_lifo() -> Worker<T>: LIFO queue
 *                  pub fn stealer(&self) -> Stealer<T>
 *                  pub fn is_empty(&self) -> bool
 *                  pub fn len(&self) -> usize
 *                  pub fn push(&self, task: T)
 *                  pub fn pop(&self) -> Option<T>
 *              Stealer: Worker队列处理
 *                  pub fn is_empty(&self) -> bool: 是否为空
 *                  pub fn steal(&self) -> Steal<T>
 *                  pub fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>
 *                  pub fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T> 转移任务
 *              pub enum Steal<T> {
 *                  Empty,
 *                  Success(T),
 *                  Retry,
 *              }
 *          ArrayQueue
 *          SegQueue
 *      Memory management:
 *          epoch
 *      Thread synchronization:
 *          channel:
 *          ShardedLock
 *          WaitGroup
 *      Utilities:
 *          Backoff
 *          CachePadded
 *          scope
 **/
