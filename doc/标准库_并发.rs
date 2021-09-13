// 
// std::thread
//      functions:
//          pub fn spawn<F, T>(f: F) -> JoinHandle<T>: 创建线程
//          pub fn sleep(dur: Duration)
//          pub fn current() -> Thread
//          pub fn park()
//      structs:
//          std::thread::JoinHandle
//              pub fn thread(&self) -> &Thread
//              pub fn join(self) -> Result<T>
//          std::thread::Thread
//              pub fn unpark(&self)
//              pub fn name(&self) -> Option<&str>
//              pub fn id(&self) -> ThreadId
//          std::thread::Builder
//              pub fn new() -> Builder
//              pub fn name(self, name: String) -> Builder
//              pub fn stack_size(self, size: usize) -> Builder
//              pub fn spawn<F, T>(self, f: F) -> Result<JoinHandle<T>>
//          std::thread::LocalKey
//              pub fn with<F, R>(&'static self, f: F) -> R
// 
// std::sync: 同步工具
//      structs:
//          std::sync::Arc: 原子引用计数器, 共享引用
//              pub fn new(data: T) -> Arc<T>
//              impl<T> Clone for Arc<T> where T: ?Sized, 
//          std::sync::Barrier: 栅栏, 同步线程
//              pub fn new(n: usize) -> Barrier
//              pub fn wait(&self) -> BarrierWaitResult
//          std::sync::Mutex: 互斥体, 同步引用
//              pub fn new(t: T) -> Mutex<T>
//              pub fn lock(&self) -> LockResult<MutexGuard<'_, T>>
//              pub fn is_poisoned(&self) -> bool
//              pub fn into_inner(self) -> LockResult<T>
//              MutexGuard:
//                  impl<T: ?Sized> Deref for MutexGuard<'_, T> return T
//                  impl<T, A> DerefMut for Box<T, A> where T: ?Sized, A: Allocator, 
//          std::sync::RwLock: 读写锁
//              pub fn new(t: T) -> RwLock<T>
//              pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>>
//              pub fn try_read(&self) -> TryLockResult<RwLockReadGuard<'_, T>>
//              pub fn write(&self) -> LockResult<RwLockWriteGuard<'_, T>>
//              pub fn try_write(&self) -> TryLockResult<RwLockWriteGuard<'_, T>>
//              pub fn is_poisoned(&self) -> bool
//              pub fn into_inner(self) -> LockResult<T>
//          std::sync::Once: 执行一次
//          std::sync::Condvar
//          std::sync::mpsc
// 
// std::sync::mpsc: Multi-producer, single-consumer FIFO queue
//      functions:
//          pub fn channel<T>() -> (Sender<T>, Receiver<T>): 异步无界Channel
//          pub fn sync_channel<T>(bound: usize) -> (SyncSender<T>, Receiver<T>): 同步有界Channel
//      std::sync::mpsc::Sender: 异步消息发送端
//          pub fn send(&self, t: T) -> Result<(), SendError<T>>
//      std::sync::mpsc::SyncSender: 同步消息发送端
//          pub fn send(&self, t: T) -> Result<(), SendError<T>>
//          pub fn try_send(&self, t: T) -> Result<(), TrySendError<T>>
//      std::sync::mpsc::Receiver: 接收端
//          pub fn recv(&self) -> Result<T, RecvError>
//          pub fn try_recv(&self) -> Result<T, TryRecvError>
//          pub fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError>
// 
//  std::sync::atomic: 原子类
//     pub enum Ordering {
//         Relaxed, // 自由顺序
//         Release,
//         Acquire,
//         AcqRel,
//         SeqCst  // 排序一致性顺序
//     }
