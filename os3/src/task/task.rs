//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // Done LAB1: Add whatever you need about the Task.
    // ## Lab1 adds
    // 记录每个系统调用的次数
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    // 记录开始时间，便于管理时间片
    pub start_time: usize,
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
