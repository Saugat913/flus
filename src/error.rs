pub type Result<T> = anyhow::Result<T>;

#[macro_export]
macro_rules! app_error {
    ($msg:expr) => {
        anyhow::anyhow!("[Error]: {}",$msg)
    };

    ($fmt:expr,$($arg:tt)*)=>{
        anyhow::anyhow!($fmt, $($arg)*)
    };
}
