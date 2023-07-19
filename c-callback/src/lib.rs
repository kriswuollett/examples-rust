static VAL: tokio::sync::OnceCell<i32> = tokio::sync::OnceCell::const_new();

async fn init_val_fn() -> i32 {
    1
}

struct StdMutexCallback {
    context: std::sync::Arc<std::sync::Mutex<*mut std::os::raw::c_void>>,
    f: extern "C" fn(context: *mut std::os::raw::c_void),
}

unsafe impl Send for StdMutexCallback {}

struct TokioMutexCallback {
    context: std::sync::Arc<tokio::sync::Mutex<*mut std::os::raw::c_void>>,
    f: extern "C" fn(context: *mut std::os::raw::c_void),
}

unsafe impl Send for TokioMutexCallback {}

#[no_mangle]
pub extern "C" fn std_mutex_init(
    context: *mut std::os::raw::c_void,
    on_complete: extern "C" fn(context: *mut std::os::raw::c_void),
) {
    let callback = StdMutexCallback {
        context: std::sync::Arc::new(std::sync::Mutex::new(context)),
        f: on_complete,
    };
    tokio::task::spawn(async move {
        println!("init task");
        let val = VAL.get_or_init(init_val_fn).await;
        println!("init done: {val}");
        let mut locked_context = callback.context.lock().unwrap();
        (callback.f)(*locked_context);
    });
}

#[no_mangle]
pub extern "C" fn tokio_mutex_init(
    context: *mut std::os::raw::c_void,
    on_complete: extern "C" fn(context: *mut std::os::raw::c_void),
) {
    let callback: TokioMutexCallback = TokioMutexCallback {
        context: std::sync::Arc::new(tokio::sync::Mutex::new(context)),
        f: on_complete,
    };
    tokio::task::spawn(async move {
        println!("init task");
        let val = VAL.get_or_init(init_val_fn).await;
        println!("init done: {val}");
        let mut locked_context = callback.context.lock().await;
        (callback.f)(*locked_context);
    });
}
