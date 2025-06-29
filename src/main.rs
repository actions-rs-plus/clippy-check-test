#[repr(C)]
struct ExternalStruct {
    value: i32,
}

unsafe extern "C" {
    fn external_call(es: *mut ExternalStruct);
}

fn main() {
    println!("Hello, world!");

    let mut es = ExternalStruct { value: 5 };

    unsafe { external_call(&mut es) };
}
