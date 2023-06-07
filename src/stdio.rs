use std::{
    cell::RefCell,
    io::{self, BufWriter, StdinLock, StdoutLock, Write},
};

thread_local! {
    static STDIN_LOCK: RefCell<StdinLock<'static>>
        = RefCell::new(Box::leak(Box::new(io::stdin())).lock());
    static STDOUT_LOCK: RefCell<BufWriter<StdoutLock<'static>>> = {
        let stdout = Box::leak(Box::new(io::stdout()));
        let lock = stdout.lock();
        RefCell::new(BufWriter::new(lock))
    };
}

pub fn with_stdout_lock<T, F: FnOnce(&mut BufWriter<StdoutLock>) -> T>(f: F) -> T {
    STDOUT_LOCK.with(|stdout| f(&mut *stdout.borrow_mut()))
}

pub fn write() {}

pub fn flush() {
    with_stdout_lock(|stdout| stdout.flush().unwrap())
}

#[macro_export]
macro_rules! p {
    ($($args:tt)*) => {
        crate::stdio::with_stdout_lock(|stdout| write!(stdout, $($args)*).unwrap())
    };
}

#[macro_export]
macro_rules! pln {
    ($($args:tt)*) => {
        crate::stdio::with_stdout_lock(|stdout| writeln!(stdout, $($args)*).unwrap())
    };
}
