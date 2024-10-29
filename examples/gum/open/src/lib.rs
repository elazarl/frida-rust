/* This example is in the public domain */

use frida_gum as gum;
use gum::{
    interceptor::{Interceptor, InvocationContext, InvocationListener},
    Gum, Module,
};
use std::os::raw::{c_int, c_void};
use std::sync::OnceLock;

struct OpenListener;

impl InvocationListener for OpenListener {
    fn on_enter(&mut self, _context: InvocationContext) {
        println!("Enter: open()");
    }

    fn on_leave(&mut self, _context: InvocationContext) {
        println!("Leave: open()");
    }
}

#[no_mangle]
extern "C" fn example_agent_main(_user_data: *const c_void, resident: *mut c_int) {
    unsafe { *resident = 1 };

    static CELL: OnceLock<Gum> = OnceLock::new();
    let gum = CELL.get_or_init(|| Gum::obtain());

    let mut interceptor = Interceptor::obtain(gum);
    let mut listener = OpenListener {};

    let module = Module::obtain(gum);
    let modules = module.enumerate_modules();
    for module in modules {
        println!(
            "{}@{:#x}/{:#x}",
            module.name, module.base_address, module.size
        );
    }

    let open = module.find_export_by_name(None, "open").unwrap();
    interceptor.attach(open, &mut listener).unwrap();
}
