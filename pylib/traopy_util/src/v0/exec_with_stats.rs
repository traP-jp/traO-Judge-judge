use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use std::collections::HashMap;

use nix::libc::{c_int, rusage, wait4};
use std::mem::MaybeUninit;
use std::process::Command;

#[derive(Clone, Debug)]
#[gen_stub_pyclass]
#[pyclass(module = "traopy_util.util.v0")]
pub struct ExecStats {
    time_ms: i64,
    memory_kib: i64,
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
}

#[derive(Clone, Debug)]
#[gen_stub_pyclass_enum]
#[pyclass(module = "traopy_util.util.v0")]
pub enum ExecResult {
    Success(ExecStats),
    Timeout(i32),
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.v0")]
/// Execute a command with environment variables and a time limit
pub async fn exec_with_stats(
    cmd: String,
    envs: HashMap<String, String>,
    time_limit_ms: i64,
) -> PyResult<ExecResult> {
    let child_proc = std::process::Command::new(cmd)
        .envs(envs)
        .spawn()
        .expect("Failed to start child process");
    let pid = child_proc.id();
    let wait = futures::future::select(
        Box::pin(wait4_child(pid as i32)),
        Box::pin(timeout(time_limit_ms)),
    )
    .await;
    match wait {
        futures::future::Either::Left((usage, _)) => {
            // Child process finished before timeout
            let time_ms = usage.ru_utime.tv_sec * 1000 + usage.ru_utime.tv_usec / 1000;
            let memory_kib = usage.ru_maxrss;
            return Ok(ExecResult::Success(ExecStats {
                time_ms: time_ms as i64,
                memory_kib: memory_kib as i64,
            }));
        }
        futures::future::Either::Right((_, _)) => {
            // Timeout occurred
            // Terminate the child process
            let _ = std::process::Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .spawn();
            return Ok(ExecResult::Timeout(pid as i32));
        }
    }
}

async fn wait4_child(pid: i32) -> rusage {
    let mut usage = MaybeUninit::<rusage>::uninit();
    let mut status = MaybeUninit::<c_int>::uninit();
    unsafe { wait4(pid, status.as_mut_ptr(), 0, usage.as_mut_ptr()) };
    let usage = unsafe { usage.assume_init() };
    usage
}

async fn timeout(time_limit_ms: i64) -> anyhow::Result<()> {
    let time_limit_sec = (time_limit_ms as f64) / 1000.0;
    let sleep_cmd = format!("sleep {}", time_limit_sec);
    let mut command = Command::new("sh");
    command.arg("-c").arg(sleep_cmd);
    command.spawn().expect("Failed to start sleep command");
    return Ok(());
}
