use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MICRO_PER_SEC: usize = 1_000_000;

// 可以取得当前 mtime 计数器的值
pub fn get_time() -> usize {
    time::read()
}

// CLOCK_FREQ 是一个预先获取到的各平台不同的时钟频率，单位是 Hz，这里先写死为 12.5MHz

// 以微秒返回计数器的值
pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}

// 对 set_timer 进行了封装， 它首先读取当前 mtime 的值，然后计算出 10ms 之内计数器的增量，再将 mtimecmp 设置为二者的和。 这样，10ms 之后一个 S 特权级时钟中断就会被触发
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}
