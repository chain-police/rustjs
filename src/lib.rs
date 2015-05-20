#![allow(non_snake_case)]
#![allow(non_uppercase_statics)]

extern crate libc;

use std::default::Default;
use std::fmt;
use std::mem;
use std::ptr;

pub use util::cmd::Commander;
pub mod util;

extern {
  fn v8_runtime(source: &[u8]) -> i32;
  fn v8_free_platform() -> bool;
  fn v8_initialize_platform() -> bool;
  fn v8_initialize() -> bool;
  fn v8_dispose() -> bool;
  fn v8_set_array_buffer_allocator() -> bool;

  fn v8_locker_is_locked() -> bool;
  fn v8_locker_is_active() -> bool;
  fn v8_locker(closure: extern fn());

  fn v8_handle_scope(closure: extern fn());

  fn v8_isolate_new();
  fn v8_isolate_dispose();
  fn v8_isolate_enter();
  fn v8_isolate_exit();

  fn v8_context_new();
  fn v8_context_enter();
  fn v8_context_exit();
  fn v8_context_global();
  fn v8_context_scope(closure: extern fn());

  fn v8_script_compile(source: &[u8]) -> Script;
  fn v8_script_run(this: &Script);

  fn v8_string_new_from_utf8(data: *const u8) -> String;
}

#[repr(C)]
pub struct Locker(*mut u8);
impl Locker {
  pub fn IsLocked() -> bool {
    unsafe { v8_locker_is_locked() }
  }
  pub fn IsActive() -> bool {
    unsafe { v8_locker_is_active() }
  }
}
pub fn with_locker(closure: extern fn()) {
  unsafe {
    v8_locker(closure);
  }
}

#[repr(C)]
pub struct HandleScope(*mut u8);
pub fn with_handle_scope(closure: extern fn()) {
  unsafe { 
    v8_handle_scope(closure);
  }
}

#[repr(C)]
pub struct Isolate(*mut Isolate);
pub fn with_isolate_scope<T>(closure: &Fn() -> T) -> T {
  V8::EnterIsolate();
  let rval = closure();
  V8::ExitIsolate();
  rval
}

#[repr(C)]
pub struct Context(*mut Context);
impl Context {
  pub fn New() {
    unsafe { v8_context_new() }
  }
  pub fn Enter() {
    unsafe { v8_context_enter() }
  }
  pub fn Exit() {
    unsafe { v8_context_exit() }
  }
  pub fn Global() {
    unsafe { v8_context_global() }
  }
}
pub fn with_context_scope(closure: extern fn()) {
  unsafe {
    v8_context_scope(closure);
  }
}

#[repr(C)]
pub struct Script(*mut *mut Script);
impl Script {
  pub fn Compile(data: &[u8]) -> Script {
    unsafe { v8_script_compile(data) }
  }
  pub fn Run(&mut self) {
    unsafe { v8_script_run(self) }
  }
}

#[repr(C)]
pub struct String(*mut *mut String);
impl String {
  pub fn NewFromUtf8(data: &str) -> String {
    unsafe { v8_string_new_from_utf8(data.as_ptr()) }
  }
}

#[repr(C)]
pub struct V8(*mut V8);
impl V8 {
  pub fn Runtime() -> i32 {
    extern fn on_locked() {
      with_isolate_scope(&|| {
        with_handle_scope(on_handle_scoped);
      });
    }
    extern fn on_handle_scoped() {
      Context::New();
      with_context_scope(on_context_scoped);
    }
    extern fn on_context_scoped() {
      let process_str = String::NewFromUtf8("process");
      let source = Commander::GetSource();
      let mut script = Script::Compile(source.as_bytes());
      script.Run();
    }

    let mut code :i32 = 1;
    V8::InitializePlatform();
    V8::Initialize();
    V8::SetArrayBufferAllocator();
    V8::NewIsolate();
    with_locker(on_locked);
    V8::DisposeIsolate();
    V8::Dispose();
    V8::UnInitializePlatform();
    return code;
  }
  pub fn UnInitializePlatform() -> bool {
    unsafe { v8_free_platform() }
  }
  pub fn InitializePlatform() -> bool {
    unsafe { v8_initialize_platform() }
  }
  pub fn Initialize() -> bool {
    unsafe { v8_initialize() }
  }
  pub fn Dispose() -> bool {
    unsafe { v8_dispose() }
  }
  pub fn SetArrayBufferAllocator() -> bool {
    unsafe { v8_set_array_buffer_allocator() }
  }
  pub fn NewIsolate() {
    unsafe { v8_isolate_new() }
  }
  pub fn DisposeIsolate() {
    unsafe { v8_isolate_dispose() }
  }
  pub fn EnterIsolate() {
    unsafe { v8_isolate_enter() }
  }
  pub fn ExitIsolate() {
    unsafe { v8_isolate_exit() }
  }
}
