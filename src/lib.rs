#![no_std]
#![allow(non_snake_case)]

use core::{ffi::c_void, hint::unreachable_unchecked, panic::PanicInfo};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! { unsafe { unreachable_unchecked() } }

#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> *mut c_void;
    fn VirtualProtect(addr: *mut c_void, size: usize, new_prot: u32, old_prot: *mut u32) -> i32;
    static HeapFree: c_void;
    static HeapAlloc: c_void;
    static HeapReAlloc: c_void;
    static HeapCreate: c_void;
}

#[unsafe(no_mangle)]
pub fn JNI_OnLoad(_jvm: usize, _reserved: usize) -> i32 {
    0x10008 // JNI_VERSION_1_8
}

#[cfg(target_os = "windows")]
#[unsafe(no_mangle)]
pub fn _DllMainCRTStartup(_inst: usize, _reason: u32, _reserved: usize) -> i32 { 1 }

#[unsafe(no_mangle)]
pub unsafe fn Java_cyb0124_NativeLoader_allocPagesRW(_jni: usize, _cls: usize, size: usize) -> *mut c_void {
    unsafe {
        #[cfg(target_os = "windows")]
        let result = VirtualAlloc(0, size, /* MEM_COMMIT */ 0x1000, /* PAGE_READWRITE */ 4);
        #[cfg(not(target_os = "windows"))]
        let result = libc::mmap(core::ptr::null_mut(), size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
        result
    }
}

#[unsafe(no_mangle)]
pub unsafe fn Java_cyb0124_NativeLoader_wrapBuffer(jni: &*const *const (), _cls: usize, addr: usize, size: i64) -> usize {
    unsafe {
        let func: unsafe extern "system" fn(&*const *const (), usize, i64) -> usize = core::mem::transmute(jni.add(229).read());
        func(jni, addr, size) // NewDirectByteBuffer
    }
}

#[unsafe(no_mangle)]
pub unsafe fn Java_cyb0124_NativeLoader_setAndRunPagesRX(
    jni: usize,
    _cls: usize,
    base: *mut c_void,
    rx_size: usize,
    table: *mut *const c_void,
    entry: unsafe extern "system" fn(usize, usize),
    arg: usize,
) {
    unsafe {
        #[cfg(target_os = "windows")]
        {
            table.write(&HeapFree);
            table.add(1).write(&HeapAlloc);
            table.add(2).write(&HeapReAlloc);
            table.add(3).write(&HeapCreate);
            VirtualProtect(base, rx_size, /* PAGE_EXECUTE_READ */ 0x20, core::mem::MaybeUninit::uninit().as_mut_ptr());
        }
        #[cfg(not(target_os = "windows"))]
        {
            table.write(core::mem::transmute(libc::free as unsafe extern "C" fn(*mut c_void)));
            table.add(1).write(core::mem::transmute(libc::malloc as unsafe extern "C" fn(usize) -> *mut c_void));
            table.add(2).write(core::mem::transmute(libc::realloc as unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void));
            table.add(3).write(core::mem::transmute(libc::posix_memalign as unsafe extern "C" fn(*mut *mut c_void, usize, usize) -> i32));
            libc::mprotect(base, rx_size, libc::PROT_READ | libc::PROT_EXEC);
        }
        entry(jni, arg)
    }
}
