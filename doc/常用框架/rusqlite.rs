// 
// rusqlite: sqlite数据库交互
//      Connection:
//          open:
//              pub fn open<P: AsRef<Path>>(path: P) -> Result<Connection>
//              pub fn open_with_flags<P: AsRef<Path>>(path: P, flags: OpenFlags) -> Result<Connection>
//              pub fn open_in_memory() -> Result<Connection>
//              pub fn open_in_memory_with_flags(flags: OpenFlags) -> Result<Connection>
//          execute:
//              pub fn prepare(&self, sql: &str) -> Result<Statement<'_>>
//              pub fn prepare_cached(&self, sql: &str) -> Result<CachedStatement<'_>>
//              pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize>
//              pub fn execute_batch(&self, sql: &str) -> Result<()>
//          transaction:
//              pub fn transaction(&mut self) -> Result<Transaction<'_>>
//          close:
//              pub fn close(self) -> Result<(), (Connection, Error)>
//          Convenient:
//              pub fn query_row<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T> where P: Params, F: FnOnce(&Row<'_>) -> Result<T>, 
//      Statement:
//          execute:
//              pub fn execute<P: Params>(&mut self, params: P) -> Result<usize>
//          result:
//              pub fn column_names(&self) -> Vec<&str>
//              pub fn column_count(&self) -> usize
//              pub fn column_index(&self, name: &str) -> Result<usize>
//          Convenient:
//              pub fn insert<P: Params>(&mut self, params: P) -> Result<i64>
//              pub fn query<P: Params>(&mut self, params: P) -> Result<Rows<'_>>
//              pub fn query_map<T, P, F>(&mut self, params: P, f: F ) -> Result<MappedRows<'_, F>> where P: Params, F: FnMut(&Row<'_>) -> Result<T>,
//              pub fn query_and_then<T, E, P, F>(&mut self, params: P, f: F) -> Result<AndThenRows<'_, F>> where P: Params, E: From<Error>, F: FnMut(&Row<'_>) -> Result<T, E>,  
//      Row:
//          pub fn get<I: RowIndex, T: FromSql>(&self, idx: I) -> Result<T>
//      Transaction:
//          pub fn commit(self) -> Result<()>
//          pub fn rollback(self) -> Result<()>
