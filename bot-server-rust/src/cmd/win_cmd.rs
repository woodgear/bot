// to get output of process there three thing should by notice.
//https://docs.microsoft.com/en-us/windows/desktop/ipc/anonymous-pipe-operations
//1. The WriteFile call does not return until it has written the specified number of bytes to the pipe or an error occurs.
//If the pipe buffer is full and there are more bytes to be written, WriteFile does not return until another process reads
//https://stackoverflow.com/questions/16070444/readfile-function-for-an-anonymous-pipe
//2. after data has been put into the pipe, you can read it out until you reach the end of the data, then reading will fail.
//3. according note2 you should close the write end of the output pipe after process end
//https://devblogs.microsoft.com/oldnewthing/20161207-00/?p=94875
//4. You won’t get ERROR_PIPE_BROKEN until all the write handles are closed.
//One common reason why you don’t get the error is that there’s still a write handle open in the parent process.
//Another possibility is that the child process launched a subprocess which inherited the write handle, or more generally,
//the handle got duplicated into another process by some means.
#![allow(dead_code)]
use std::{
    ffi::{OsStr, OsString},
    mem,
    os::windows::io::{AsRawHandle, FromRawHandle, RawHandle},
    ptr::{self, null_mut},
    time::*,
};

use encoding::{all, DecoderTrap, EncodingRef};
use failure::{self, err_msg, format_err, Error};
use log::*;
use std::iter;
use std::os::windows::ffi::OsStrExt;
use winapi::{
    shared::{minwindef::*, ntdef::NULL, winerror::*},
    um::{
        fileapi::ReadFile,
        handleapi::{CloseHandle, SetHandleInformation, INVALID_HANDLE_VALUE},
        minwinbase::*,
        namedpipeapi::*,
        processthreadsapi::{self, *},
        synchapi::*,
        userenv::{CreateEnvironmentBlock, DestroyEnvironmentBlock},
        winbase::*,
        winnt::*,
        winuser::{SW_HIDE, SW_SHOW},
    },
};

use winapi::um::errhandlingapi::GetLastError;

pub fn get_last_err() -> u32 {
    unsafe { GetLastError() }
}

pub fn to_nullterm<S: AsRef<OsStr>>(s: S) -> Vec<u16> {
    let s = s.as_ref();
    s.encode_wide().chain(iter::once(0_u16)).collect()
}

pub fn close_handle(handle: HANDLE) {
    unsafe {
        if handle != INVALID_HANDLE_VALUE {
            let ret = CloseHandle(handle);
            if ret == 0 {
                error!("============> CloseHandle error {}", get_last_err());
            }
        }
    }
}

pub fn to_string(bin: Vec<u8>) -> Result<String, Error> {
    if bin.is_empty() {
        return Ok("".to_string());
    }

    let decodelist = [
        all::GBK as EncodingRef,
        all::UTF_8 as EncodingRef,
        all::UTF_16LE as EncodingRef,
    ];

    for decoder in decodelist.iter() {
        if let Ok(res) = decoder.decode(&bin, DecoderTrap::Strict) {
            return Ok(res);
        }
    }
    return Err(err_msg("parse to string fail"));
}

pub fn check(status: BOOL) -> Result<(), Error> {
    if status == 0 {
        Err(format_err!("get_last_err {}", get_last_err()))
    } else {
        Ok(())
    }
}

const ERROR_BROKEN_PIPE: u32 = 109;

struct ReadWritePipe {
    read: PVOID,
    write: PVOID,
    read_has_close: bool,
    write_has_close: bool,
}

enum PipeReadEnum {
    End(Vec<u8>),
    Continute(Vec<u8>),
}

impl ReadWritePipe {
    pub fn new() -> Result<Self, Error> {
        //if we want to child proccess to use parent process handle bInheritHandle must be true
        //in this case we want to child process write data to this pipe so bInheritHandle must be true
        //https://bbs.csdn.net/topics/50473554
        let mut attributes = SECURITY_ATTRIBUTES {
            nLength: mem::size_of::<SECURITY_ATTRIBUTES>() as DWORD,
            lpSecurityDescriptor: ptr::null_mut(),
            bInheritHandle: true as BOOL,
        };
        let (mut read, mut write) = (ptr::null_mut(), ptr::null_mut());
        if FALSE
            == unsafe {
                CreatePipe(
                    &mut read as PHANDLE,
                    &mut write as PHANDLE,
                    &mut attributes as LPSECURITY_ATTRIBUTES,
                    1024 * 1024,
                )
            }
        {
            return Err(format_err!("create pipe fail {}", get_last_err()));
        }

        //please see note4
        let ret = unsafe { SetHandleInformation(read, HANDLE_FLAG_INHERIT, 0) };
        if ret == 0 {
            return Err(format_err!(
                "create pipe SetHandleInformation fail {}",
                get_last_err()
            ));
        }

        Ok(Self {
            read,
            write,
            read_has_close: false,
            write_has_close: false,
        })
    }

    pub fn try_read(&mut self) -> Result<PipeReadEnum, Error> {
        let mut all_buff: Vec<u8> = vec![];
        loop {
            let mut data_avail = 0;
            if unsafe {
                PeekNamedPipe(
                    self.read,
                    NULL as LPVOID,
                    0,
                    NULL as LPDWORD,
                    &mut data_avail,
                    NULL as LPDWORD,
                )
            } == 0
            {
                if get_last_err() == ERROR_BROKEN_PIPE {
                    return Ok(PipeReadEnum::End(all_buff));
                }
                return Err(format_err!("PeekNamedPipe fail {}", get_last_err()));
            }

            if data_avail == 0 {
                return Ok(PipeReadEnum::Continute(all_buff));
            }

            let mut data_readed = 0;
            let mut buff_temp: Vec<u8> = vec![0 as u8; data_avail as usize];
            let read_ret = unsafe {
                ReadFile(
                    self.read,
                    buff_temp.as_mut_ptr() as LPVOID,
                    data_avail,
                    &mut data_readed,
                    NULL as LPOVERLAPPED,
                )
            };
            if read_ret == 0 || data_readed == 0 || data_avail != data_readed {
                return Err(format_err!(
                    "ReadFile Err data_avail {} data_readed {} err_code {}",
                    data_avail,
                    data_readed,
                    get_last_err()
                ));
            }
            all_buff.extend_from_slice(&buff_temp);
        }
    }

    pub fn get_write_side(&self) -> PVOID {
        return self.write;
    }

    pub fn close_write(&mut self) {
        if !self.write_has_close {
            close_handle(self.write);
            self.write_has_close = true;
        }
    }
}

impl Drop for ReadWritePipe {
    fn drop(&mut self) {
        if !self.read_has_close {
            close_handle(self.read);
        }

        if !self.write_has_close {
            close_handle(self.write);
        }
    }
}

struct EnvBlock {
    env_ptr: LPVOID,
}

impl EnvBlock {
    fn new(token: HANDLE, b_inherit: BOOL) -> Result<Self, Error> {
        let mut env_ptr: LPVOID = ptr::null_mut();
        if FALSE == unsafe { CreateEnvironmentBlock(&mut env_ptr, token, b_inherit) } {
            return Err(format_err!(
                "CreateEnvironmentBlock fail {}",
                get_last_err()
            ));
        }
        return Ok(Self { env_ptr });
    }
}

impl Drop for EnvBlock {
    fn drop(&mut self) {
        if FALSE == unsafe { DestroyEnvironmentBlock(self.env_ptr) } {
            error!("DestroyEnvironmentBlock fail {}", get_last_err())
        }
    }
}

#[derive(Debug)]
pub struct WaitConfig {
    pub output: bool,
}

impl Default for WaitConfig {
    fn default() -> Self {
        Self { output: false }
    }
}

#[derive(Debug)]
pub struct TimeoutConfig(pub Option<Duration>);

impl From<u32> for TimeoutConfig {
    fn from(secs: u32) -> Self {
        Self(Some(Duration::from_secs(u64::from(secs))))
    }
}

impl From<Duration> for TimeoutConfig {
    fn from(dur: Duration) -> Self {
        Self(Some(dur))
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self(Some(Duration::from_secs(60 * 2))) //default to 2 minutes, it should enough for all cmd
    }
}

#[allow(non_snake_case)]
struct CreateProcessConfig {
    token: Option<HANDLE>,
    app_name: Option<OsString>,
    cmd_line: Option<OsString>,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpCurrentDirectory: LPCWSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    process_info: PROCESS_INFORMATION,
    env: Option<EnvBlock>,
}

impl Default for CreateProcessConfig {
    fn default() -> Self {
        Self {
            token: None,
            app_name: None,
            cmd_line: None,
            lpProcessAttributes: null_mut(),
            lpThreadAttributes: null_mut(),
            bInheritHandles: true as BOOL,
            dwCreationFlags: 0,
            env: None,
            lpCurrentDirectory: null_mut(),
            lpStartupInfo: null_mut(),
            process_info: unsafe { mem::zeroed() },
        }
    }
}

impl CreateProcessConfig {
    fn start_create_process(&mut self) -> Result<(Handle, u64), Error> {
        let env = self.env.as_mut().map_or(ptr::null_mut(), |e| e.env_ptr);
        let mut app_name = self.app_name.clone().map(to_nullterm);
        let mut cmd_line = self.cmd_line.clone().map(to_nullterm);
        let app_name_ptr = app_name
            .as_mut()
            .map_or(ptr::null_mut(), |d| d.as_mut_ptr());
        let cmd_line_ptr = cmd_line
            .as_mut()
            .map_or(ptr::null_mut(), |d| d.as_mut_ptr());

        let ret = if let Some(token) = self.token {
            unsafe {
                CreateProcessAsUserW(
                    token,
                    app_name_ptr,
                    cmd_line_ptr,
                    self.lpProcessAttributes,
                    self.lpThreadAttributes,
                    self.bInheritHandles,
                    self.dwCreationFlags,
                    env,
                    self.lpCurrentDirectory,
                    self.lpStartupInfo,
                    &mut self.process_info,
                )
            }
        } else {
            unsafe {
                CreateProcessW(
                    app_name_ptr,
                    cmd_line_ptr,
                    self.lpProcessAttributes,
                    self.lpThreadAttributes,
                    self.bInheritHandles,
                    self.dwCreationFlags,
                    env,
                    self.lpCurrentDirectory,
                    self.lpStartupInfo,
                    &mut self.process_info,
                )
            }
        };
        warn!("start_create_process {}", ret);
        if ret == 0 {
            return Err(format_err!("start_create_process fail {}", get_last_err()));
        }
        unsafe {
            mem::drop(Handle::from_raw_handle(self.process_info.hThread));
            return Ok((
                Handle::from_raw_handle(self.process_info.hProcess),
                u64::from(self.process_info.dwProcessId),
            ));
        }
    }
}

#[derive(Default, Debug)]
pub struct Exec {
    pub cmd_line: OsString,
    pub show_window: bool,
    pub timeout: TimeoutConfig,
    pub disable_wow64_redirect_on_x64: bool,
    pub as_user: bool, //run with user token
    pub break_away_from_job: bool,
    pub create_new_process_group: bool,
}

impl Exec {
    pub fn new<S: AsRef<OsStr>>(cmd_line: S) -> Self {
        Self {
            cmd_line: cmd_line.as_ref().to_os_string(),
            ..Default::default()
        }
    }

    pub fn show_window(mut self) -> Self {
        self.show_window = true;
        return self;
    }

    pub fn cancel_timeout(mut self) -> Self {
        self.timeout = TimeoutConfig(None);
        return self;
    }

    pub fn set_timeout<T: Into<TimeoutConfig>>(mut self, timeout: T) -> Self {
        self.timeout = timeout.into();
        return self;
    }

    pub fn disable_wow64_redirect_on_x64(mut self) -> Self {
        self.disable_wow64_redirect_on_x64 = true;
        return self;
    }

    pub fn break_away_from_job(mut self) -> Self {
        self.break_away_from_job = true;
        return self;
    }
    pub fn create_new_process_group(mut self) -> Self {
        self.create_new_process_group = true;
        return self;
    }
}

impl Exec {
    pub fn run(&self) -> Result<(), Error> {
        Exec::create_process(self, None, None, None)?;
        Ok(())
    }

    pub fn wait_util_end(&self) -> Result<(), Error> {
        let (handle, _pid) = Exec::create_process(self, None, None, None)?;
        let timeout = self
            .timeout
            .0
            .map(|d| d.as_millis() as u32)
            .unwrap_or_else(|| INFINITE as u32);

        match unsafe { WaitForSingleObject(handle.as_raw_handle(), timeout) } {
            WAIT_ABANDONED => {
                return Err(format_err!(
                    "{:?} wait_util_end WaitForSingleObject WAIT_ABANDONED",
                    self
                ));
            }
            WAIT_OBJECT_0 => return Ok(()),
            WAIT_TIMEOUT => {
                return Err(format_err!(
                    "{:?} wait_util_end WaitForSingleObject WAIT_TIMEOUT",
                    self
                ));
            }
            WAIT_FAILED => {
                return Err(format_err!(
                    "{:?} wait_util_end WaitForSingleObject WAIT_FAILED {}",
                    self,
                    get_last_err()
                ));
            }
            other => {
                return Err(format_err!(
                    "WaitForSingleObject fail other {:?} {} {}",
                    self,
                    other,
                    get_last_err()
                ));
            }
        }
    }

    pub fn get_raw_output(&self) -> Result<Vec<u8>, Error> {
        let mut pipe = ReadWritePipe::new()?;
        let (handle, _pid) = Exec::create_process(self, None, Some(pipe.get_write_side()), None)?;
        let now = Instant::now();
        let mut all_buf = vec![];
        let mut process_ended = false;
        let read_delay_mills = 1000;
        let mut is_process_success = true;
        let mut exit_code: DWORD = 0;
        loop {
            if process_ended {
                std::thread::sleep(Duration::from_millis(read_delay_mills.into()));
            } else {
                let wait_ret =
                    unsafe { WaitForSingleObject(handle.as_raw_handle(), read_delay_mills) };
                match wait_ret {
                    WAIT_ABANDONED => {
                        return Err(err_msg("WaitForSingleObject WAIT_ABANDONED"));
                    }
                    WAIT_OBJECT_0 => {
                        unsafe {
                            GetExitCodeProcess(handle.as_raw_handle(), &mut exit_code);
                        }
                        if exit_code != 0 {
                            is_process_success = false;
                        }
                        process_ended = true;
                        pipe.close_write();
                    }
                    WAIT_TIMEOUT => {}
                    WAIT_FAILED => {
                        return Err(format_err!("WaitForSingleObject fail {}", get_last_err()));
                    }
                    other => {
                        return Err(format_err!(
                            "WaitForSingleObject fail other {} {}",
                            other,
                            get_last_err()
                        ));
                    }
                };
            }
            match pipe.try_read()? {
                PipeReadEnum::Continute(data) => {
                    all_buf.extend_from_slice(&data);
                }
                PipeReadEnum::End(data) => {
                    all_buf.extend_from_slice(&data);
                    break;
                }
            };

            if let Some(timeout) = self.timeout.0 {
                let elapsed = now.elapsed();
                if elapsed > timeout {
                    return Err(format_err!("time out"));
                }
            }
        }

        if !is_process_success {
            return Err(format_err!("create process failed {}", exit_code));
        }
        return Ok(all_buf);
    }

    pub fn get_string_output(&self) -> Result<String, Error> {
        let bin = self.get_raw_output()?;
        to_string(bin)
    }

    pub fn create_process(
        config: &Exec,
        stdin: Option<RawHandle>,
        stdout: Option<RawHandle>,
        stderr: Option<RawHandle>,
    ) -> Result<(Handle, u64), Error> {
        let w_show_window = if config.show_window { SW_SHOW } else { SW_HIDE };

        let mut sinfo: STARTUPINFOW = {
            let mut sinfo: STARTUPINFOW = unsafe { mem::zeroed() };

            sinfo.cb = mem::size_of::<STARTUPINFOW>() as DWORD;
            sinfo.hStdInput = stdin.unwrap_or(ptr::null_mut());
            sinfo.hStdOutput = stdout.unwrap_or(ptr::null_mut());
            sinfo.hStdError = stderr.unwrap_or(ptr::null_mut());
            sinfo.dwFlags = STARTF_USESTDHANDLES;

            sinfo.dwFlags = STARTF_USESHOWWINDOW | STARTF_USESTDHANDLES;
            sinfo.wShowWindow = w_show_window as u16;
            sinfo
        };

        //https://docs.microsoft.com/en-us/windows/desktop/procthread/process-creation-flags
        let create_flags = {
            let mut create_flags = CREATE_UNICODE_ENVIRONMENT;
            if config.break_away_from_job {
                create_flags |= CREATE_BREAKAWAY_FROM_JOB;
            }
            if config.create_new_process_group {
                create_flags |= CREATE_NEW_PROCESS_GROUP;
            }
            create_flags
        };
        let mut create_process_config = CreateProcessConfig::default();
        create_process_config.cmd_line = Some(config.cmd_line.clone());
        create_process_config.dwCreationFlags = create_flags;
        create_process_config.lpStartupInfo = &mut sinfo;

        let (process_handle, pid) = create_process_config.start_create_process()?;

        return Ok((process_handle, pid));
    }
}

#[derive(Debug)]
pub struct Handle(RawHandle);

unsafe impl Send for Handle {}

impl Drop for Handle {
    fn drop(&mut self) {
        close_handle(self.as_raw_handle());
    }
}

impl AsRawHandle for Handle {
    fn as_raw_handle(&self) -> RawHandle {
        self.0
    }
}

impl FromRawHandle for Handle {
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self(handle)
    }
}

pub fn get_process_exit_code(handle: &Handle) -> Result<u32, Error> {
    let mut exit_code: u32 = 0;
    check(unsafe {
        processthreadsapi::GetExitCodeProcess(handle.as_raw_handle(), &mut exit_code as *mut u32)
    })?;
    Ok(exit_code)
}

mod exec_wrapper {
    use super::*;
    pub fn exec<S: AsRef<OsStr>>(cmd_line: S) -> Result<String, Error> {
        exec_with_timeout(cmd_line, 60 * 2)
    }

    pub fn exec_with_timeout<S: AsRef<OsStr>>(cmd_line: S, secs: u32) -> Result<String, Error> {
        Exec::new(cmd_line).set_timeout(secs).get_string_output()
    }

    pub fn exec_without_wait<S: AsRef<OsStr>>(cmd_line: S) -> Result<(), Error> {
        // Exec::new(cmd_line).create_new_process_group().run()?;
        Exec::new(cmd_line).run()?;
        Ok(())
    }

    pub fn exec_wait_without_output_timeout<S: AsRef<OsStr>>(
        cmd_line: S,
        secs: u32,
    ) -> Result<String, Error> {
        Exec::new(cmd_line).set_timeout(secs).wait_util_end()?;
        Ok("".to_string())
    }
}

pub use self::exec_wrapper::*;

#[cfg(test)]
mod test {
    use super::*;

    fn create_file_with_size(name: &str, size: u32) {
        use std::path::Path;
        let parent = Path::new(name).parent().unwrap();

        std::fs::create_dir_all(parent).unwrap();
        if let Ok(f) = std::fs::metadata(name) {
            if f.len() == size as u64 {
                return;
            } else {
                std::fs::remove_file(name).unwrap();
            }
        }
        let buf = std::iter::repeat("X")
            .take(size as usize)
            .collect::<String>();
        std::fs::write(name, buf).unwrap();
    }

    #[test]
    fn test_command_exec_fail() {
        let res = exec("escho test".to_string());
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_command_exec_success() {
        let res = exec(r#"cmd.exe /c "echo hello""#);
        assert_eq!(res.unwrap().contains("hello"), true);
    }

    #[ignore]
    #[test]
    fn test_command_never_end() {
        let now = Instant::now();
        let res = exec_with_timeout(r#"cmd.exe /c  test_scripts\test_command_never_end.bat"#, 5);
        info!("res {:?}", res);
        let err = res.unwrap_err().to_string();
        assert_eq!(err.contains("time out"), true);
        assert_eq!(now.elapsed() > Duration::from_secs(5), true);
    }

    #[ignore]
    #[test]
    fn test_command_mutli_thread() {
        let a = std::thread::spawn(|| {
            let res = exec(r#"cmd.exe /c "echo hello""#);
            assert_eq!(res.unwrap().contains("hello"), true);
        });

        let b = std::thread::spawn(|| {
            let res =
                exec_with_timeout(r#"cmd.exe /c  test_scripts\test_command_never_end.bat"#, 1);
            let err = res.unwrap_err().to_string();
            assert_eq!(err.contains("time out"), true);
        });
        let _ = a.join();
        let _ = b.join();
    }

    #[test]
    fn test_outout_big_buffer() {
        let s_1kb = "test_data\\1kb-byte-file.txt";
        let s_10kb = "test_data\\10kb-file.txt";
        let s_1mb = "test_data\\1mb-file.txt";

        create_file_with_size(s_1kb, 1010);
        create_file_with_size(s_10kb, 9770);
        create_file_with_size(s_1mb, 1_222_144);

        let res = exec_with_timeout(format!(r#"cmd.exe /c "type {}"#, s_1kb), 10).unwrap();
        assert_eq!(res.len(), 1010);
        let res = exec_with_timeout(format!(r#"cmd.exe /c "type {}"#, s_10kb), 10).unwrap();
        assert_eq!(res.len(), 9770);
        let res = exec_with_timeout(format!(r#"cmd.exe /c "type {}"#, s_1mb), 10).unwrap();
        assert_eq!(res.len(), 1_222_144)
    }

    #[test]
    fn test_outout_big_buffer_timeout() {
        let s_1mb = "test_data\\1mb-file.txt";
        create_file_with_size(s_1mb, 1_222_144);
        let now = Instant::now();
        let err = exec_with_timeout(format!(r#"cmd.exe /c "type {}"#, s_1mb), 1)
            .unwrap_err()
            .to_string();
        assert_eq!(err.contains("time out"), true);
        assert_eq!(now.elapsed() < Duration::from_secs(3), true);
    }

    #[ignore]
    #[test]
    fn test_output_slow_timeout() {
        let err = exec_with_timeout(r#"cmd.exe /c test_scripts\test_slow_6.bat"#, 3)
            .unwrap_err()
            .to_string();
        assert_eq!(err.contains("time out"), true);
    }

    #[ignore]
    #[test]
    fn test_output_slow() {
        let now = Instant::now();
        let ret = exec_with_timeout(r#"cmd.exe /c  test_scripts\test_slow_6.bat"#, 10);
        let data = ret.unwrap();
        assert_eq!(data.contains("\"1\"\r\n\"2\"\r\n"), true);
        assert_eq!(now.elapsed() > Duration::from_secs(5), true);
    }
}
