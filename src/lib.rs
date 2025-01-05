#![no_std]
#![allow(non_snake_case)]

use core::{ffi::c_void, hint::unreachable_unchecked, panic::PanicInfo};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! { unsafe { unreachable_unchecked() } }

#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
extern "system" {
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> *mut c_void;
    fn VirtualProtect(addr: *mut c_void, size: usize, new_prot: u32, old_prot: *mut u32) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "ucrt")]
extern "system" {
    static _aligned_free: c_void;
    static _aligned_malloc: c_void;
    static _aligned_realloc: c_void;
}

#[no_mangle]
pub fn JNI_OnLoad(_jvm: usize, _reserved: usize) -> i32 {
    0x10008 // JNI_VERSION_1_8
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub fn _DllMainCRTStartup(_inst: usize, _reason: u32, _reserved: usize) -> i32 { 1 }

#[no_mangle]
pub unsafe fn Java_cyb0124_NativeLoader_allocPagesRW(_jni: usize, _cls: usize, size: usize) -> *mut c_void {
    #[cfg(target_os = "windows")]
    let result = VirtualAlloc(0, size, /* MEM_COMMIT */ 0x1000, /* PAGE_READWRITE */ 4);
    #[cfg(not(target_os = "windows"))]
    let result = libc::mmap(core::ptr::null_mut(), size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
    result
}

#[no_mangle]
pub unsafe fn Java_cyb0124_NativeLoader_wrapBuffer(jni: &*const *const (), _cls: usize, addr: usize, size: i64) -> usize {
    let func: unsafe extern "system" fn(&*const *const (), usize, i64) -> usize = core::mem::transmute(jni.offset(229).read());
    func(jni, addr, size) // NewDirectByteBuffer
}

#[no_mangle]
pub unsafe fn Java_cyb0124_NativeLoader_setAndRunPagesRX(
    jni: usize,
    _cls: usize,
    base: *mut c_void,
    rx_size: usize,
    table: *mut *const c_void,
    entry: unsafe extern "system" fn(usize, usize),
    arg: usize,
) {
    #[cfg(target_os = "windows")]
    {
        table.write(&_aligned_free);
        table.offset(1).write(&_aligned_malloc);
        table.offset(2).write(&_aligned_realloc);
        VirtualProtect(base, rx_size, /* PAGE_EXECUTE_READ */ 0x20, core::mem::MaybeUninit::uninit().as_mut_ptr());
    }
    #[cfg(not(target_os = "windows"))]
    {
        table.write(core::mem::transmute(libc::free as unsafe extern "C" fn(*mut c_void)));
        table.offset(1).write(core::mem::transmute(libc::malloc as unsafe extern "C" fn(usize) -> *mut c_void));
        table.offset(2).write(core::mem::transmute(libc::realloc as unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void));
        table.offset(3).write(core::mem::transmute(libc::posix_memalign as unsafe extern "C" fn(*mut *mut c_void, usize, usize) -> i32));
        libc::mprotect(base, rx_size, libc::PROT_READ | libc::PROT_EXEC);
    }
    entry(jni, arg)
}
