# examples-rust-c-callback

Working on fixing this compile error:

```
error[E0277]: `*mut c_void` cannot be sent between threads safely
   --> src/lib.rs:30:24
    |
30  |       tokio::task::spawn(async move {
    |  _____------------------_^
    | |     |
    | |     required by a bound introduced by this call
31  | |         println!("init task");
32  | |         let val = VAL.get_or_init(init_val_fn).await;
33  | |         println!("init done: {val}");
34  | |         let mut locked_context = callback.context.lock().unwrap();
35  | |         (callback.f)(*locked_context);
36  | |     });
    | |_____^ `*mut c_void` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for `*mut c_void`
    = note: required for `std::sync::Mutex<*mut c_void>` to implement `Sync`
    = note: required for `Arc<std::sync::Mutex<*mut c_void>>` to implement `Send`
note: required because it's used within this `async` block
   --> src/lib.rs:30:24
    |
30  |       tokio::task::spawn(async move {
    |  ________________________^
31  | |         println!("init task");
32  | |         let val = VAL.get_or_init(init_val_fn).await;
33  | |         println!("init done: {val}");
34  | |         let mut locked_context = callback.context.lock().unwrap();
35  | |         (callback.f)(*locked_context);
36  | |     });
    | |_____^
note: required by a bound in `tokio::spawn`
   --> /Users/kris/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/task/spawn.rs:166:21
    |
164 |     pub fn spawn<T>(future: T) -> JoinHandle<T::Output>
    |            ----- required by a bound in this function
165 |     where
166 |         T: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`

error[E0277]: `*mut c_void` cannot be sent between threads safely
   --> src/lib.rs:48:24
    |
48  |       tokio::task::spawn(async move {
    |  _____------------------_^
    | |     |
    | |     required by a bound introduced by this call
49  | |         println!("init task");
50  | |         let val = VAL.get_or_init(init_val_fn).await;
51  | |         println!("init done: {val}");
52  | |         let mut locked_context = callback.context.lock().await;
53  | |         (callback.f)(*locked_context);
54  | |     });
    | |_____^ `*mut c_void` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for `*mut c_void`
    = note: required for `tokio::sync::Mutex<*mut c_void>` to implement `Sync`
    = note: required for `Arc<tokio::sync::Mutex<*mut c_void>>` to implement `Send`
note: required because it's used within this `async` block
   --> src/lib.rs:48:24
    |
48  |       tokio::task::spawn(async move {
    |  ________________________^
49  | |         println!("init task");
50  | |         let val = VAL.get_or_init(init_val_fn).await;
51  | |         println!("init done: {val}");
52  | |         let mut locked_context = callback.context.lock().await;
53  | |         (callback.f)(*locked_context);
54  | |     });
    | |_____^
note: required by a bound in `tokio::spawn`
   --> /Users/kris/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/task/spawn.rs:166:21
    |
164 |     pub fn spawn<T>(future: T) -> JoinHandle<T::Output>
    |            ----- required by a bound in this function
165 |     where
166 |         T: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `examples_rust_c_callback` (lib) due to 2 previous errors
```