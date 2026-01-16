use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use std::collections::HashMap;

use nix::libc::{SIGKILL, WEXITSTATUS, WIFEXITED, c_int, kill, rusage, wait4};
use nix::unistd::Pid;
use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};
#[cfg(not(target_os = "macos"))]
const RUSAGE_MAXRSS_IS_KIB: bool = true;
#[cfg(target_os = "macos")]
const RUSAGE_MAXRSS_IS_KIB: bool = false;

#[derive(Clone, Debug)]
#[gen_stub_pyclass]
#[pyclass(module = "traopy_util.util.v0")]
pub struct ExecStats {
    time_ms: i64,
    memory_kib: i64,
    exit_code: i32,
}

#[gen_stub_pymethods]
#[pymethods]
impl ExecStats {
    #[getter]
    fn time_ms(&self) -> i64 {
        self.time_ms
    }

    #[getter]
    fn memory_kib(&self) -> i64 {
        self.memory_kib
    }

    #[getter]
    fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.v0")]
/// Execute a command with environment variables and a time limit
pub async fn exec_with_stats(
    cmd: String,
    envs: HashMap<String, String>,
    time_limit_ms: i64,
) -> PyResult<Option<ExecStats>> {
    let child_proc = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .envs(envs)
        .spawn()
        .expect("Failed to start child process");
    let pid = Pid::from_raw(child_proc.id() as i32);

    let finished = Arc::new(Mutex::new(false));
    let finished_clone = Arc::clone(&finished);

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(
            (time_limit_ms + 200) as u64,
        ));

        // If finished flag is locked, this means the process has already completed
        if let Ok(mut done) = finished_clone.lock() {
            if *done {
                return;
            } else {
                let _ = unsafe { kill(pid.as_raw(), SIGKILL) };
                *done = true;
            }
        }
    });
    let pid = child_proc.id() as i32;
    let mut usage = MaybeUninit::<rusage>::uninit();
    let mut status = MaybeUninit::<c_int>::uninit();
    unsafe { wait4(pid, status.as_mut_ptr(), 0, usage.as_mut_ptr()) };
    // Set finished flag to true
    // If the flag is already locked, this means the timeout thread has already executed
    let mut killed_estimated_exec_time = None;
    {
        let mut done = finished.lock().unwrap();
        if *done {
            killed_estimated_exec_time = Some(time_limit_ms + 200);
        } else {
            *done = true;
        }
    }
    let status = unsafe { status.assume_init() };
    let exit_code = if WIFEXITED(status) {
        WEXITSTATUS(status)
    } else {
        -1
    };
    let (mut memory_used, cpu_time_elapsed) = unsafe {
        let usage = usage.assume_init();
        (
            usage.ru_maxrss as i64,
            usage.ru_utime.tv_sec as f64
                + usage.ru_stime.tv_sec as f64
                + (usage.ru_utime.tv_usec as f64 + usage.ru_stime.tv_usec as f64) * 1e-6,
        )
    };
    if !RUSAGE_MAXRSS_IS_KIB {
        memory_used /= 1024;
    }
    Ok({
        Some(ExecStats {
            time_ms: killed_estimated_exec_time.unwrap_or((cpu_time_elapsed * 1000.0) as i64),
            memory_kib: memory_used,
            exit_code,
        })
    })
}
