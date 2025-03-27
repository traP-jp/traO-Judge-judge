use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use std::collections::HashMap;

use nix::libc::{c_int, kill, rusage, wait4, SIGKILL, WEXITSTATUS, WIFEXITED};
use std::mem::MaybeUninit;

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
    let start_time = std::time::Instant::now();
    let child_proc = std::process::Command::new("timeout")
        .arg(format!("{}s", time_limit_ms as f64 / 1000.0))
        .arg("sh")
        .arg("-c")
        .arg(cmd)
        .envs(envs)
        .spawn()
        .expect("Failed to start child process");
    let pid = child_proc.id() as i32;
    let mut usage = MaybeUninit::<rusage>::uninit();
    let mut status = MaybeUninit::<c_int>::uninit();
    unsafe { wait4(pid, status.as_mut_ptr(), 0, usage.as_mut_ptr()) };
    let status = unsafe { status.assume_init() };
    let exit_code = if WIFEXITED(status) {
        WEXITSTATUS(status)
    } else {
        -1
    };
    let mut memory_used = unsafe { usage.assume_init().ru_maxrss };
    if !RUSAGE_MAXRSS_IS_KIB {
        memory_used /= 1024;
    }
    Ok({
        Some(ExecStats {
            time_ms: start_time.elapsed().as_millis() as i64,
            memory_kib: memory_used,
            exit_code,
        })
    })
}
