use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
  static THREAD_LOCAL_DATA: RefCell<HashMap<u32, u32>> = RefCell::new(HashMap::new());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_thread_local_data(key: u32, value: u32) {
    THREAD_LOCAL_DATA.with(|data| {
        data.borrow_mut().insert(key, value);
    });
}
