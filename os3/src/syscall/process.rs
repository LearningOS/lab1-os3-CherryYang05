//! Process management syscalls

use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, get_task_status, get_syscall_times, get_start_time};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// TaskInfo 结构体
/// status 记录任务状态
/// syscall_times 记录每个系统调用的次数
/// time 记录任务被调度时候的时间，方便与 TaskControlBlock 中的 starttime相减获得时间片
pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// Done YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    unsafe {
        *ti = TaskInfo {
            status: get_task_status(),
            syscall_times: get_syscall_times(),
            time: (get_time_us() - get_start_time()) / 1000
        }
    }
    0
}
