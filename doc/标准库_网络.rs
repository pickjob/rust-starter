// 
// std::net
//      udp:
//          std::net::UdpSocket
//              pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<UdpSocket>
//              pub fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], addr: A) -> Result<usize>
//              pub fn peek_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>
//              pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>
//              pub fn broadcast(&self) -> Result<bool>
//      tcp:
//          std::net::TcpListener
//              pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<TcpListener>
//              pub fn try_clone(&self) -> Result<TcpListener>
//              pub fn accept(&self) -> Result<(TcpStream, SocketAddr)>
//              pub fn incoming(&self) -> Incoming<'_>
//          std::net::TcpStream
//              pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpStream>
//              pub fn try_clone(&self) -> Result<TcpStream>
//              pub fn peek(&self, buf: &mut [u8]) -> Result<usize>
//              pub fn shutdown(&self, how: Shutdown) -> Result<()>
//              impl Read for TcpStream
//              impl Write for TcpStream
//      std::net::SocketAddr
//          pub enum SocketAddr {
//              V4(SocketAddrV4),
//              V6(SocketAddrV6),
//          }
//              pub fn new(ip: IpAddr, port: u16) -> SocketAddr
