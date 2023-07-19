static VAL: tokio::sync::OnceCell<i32> = tokio::sync::OnceCell::const_new();

async fn init_val_fn() -> i32 {
    1
}

struct Callback {
    context: *mut std::os::raw::c_void,
    on_complete: extern "C" fn(context: *mut std::os::raw::c_void),
}

impl Callback {
    fn context_as_ptr(&self) -> *mut std::os::raw::c_void {
        self.context
    }
}

unsafe impl Send for Callback {}

#[no_mangle]
pub extern "C" fn app_init(
    context: *mut std::os::raw::c_void,
    on_complete: extern "C" fn(context: *mut std::os::raw::c_void),
) {
    let callback = Callback {
        context,
        on_complete,
    };
    tokio::task::spawn(async move {
        println!("init task");
        let val = VAL.get_or_init(init_val_fn).await;
        println!("init done: {val}");
        (callback.on_complete)(callback.context_as_ptr());
    });
}
