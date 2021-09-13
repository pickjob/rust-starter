// 
// std:env: 环境信息
//      function(*_os 返回OsString, 系统特定字符串, 默认返回String):
//          运行参数:
//              args / args_os
//          环境变量:
//              var / var_os
//              vars / vars_os
//              set_var
//              remove_var
//          当前目录:
//              current_dir / set_current_dir
//              temp_dir
//              current_exe
//          PATH相关:
//              join_paths
//              split_paths
// 
// std::process::Command: 与其他进程交互
//      pub fn new<S: AsRef<OsStr>>(program: S) -> Command
//      pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Command
//      pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Command
//      pub fn env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Command
//      pub fn env_clear(&mut self) -> &mut Command
//      pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Command
//      pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command
//      pub fn args<I, S>(&mut self, args: I) -> &mut Command
//      pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command
//      pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command
//      pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command
//      pub fn spawn(&mut self) -> Result<Child>
//      pub fn status(&mut self) -> Result<ExitStatus>
//      pub fn output(&mut self) -> Result<Output>
