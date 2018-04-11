use std::ptr::null;
use std::sync::Arc;
use libc::{c_void, uint32_t};

use bonmgr::BON_MGR;
use adapter::{JSStatus, JS, JSValueType, JSType, njsc_vm_status_switch};

//调用NativeObject函数
#[no_mangle]
pub extern "C" fn native_object_function_call(
    vm: *const c_void, 
    hash: uint32_t, 
    args_size: uint32_t, 
    args_type: *const c_void,
    args: *const c_void) -> *const c_void {
        let reply: Option<JSType>;
        let js = unsafe { Arc::new(JS::new(vm)) };
        let vec = args_to_vec(vm, args_size, args_type as *const u8, args as *const u64);
        //同步块
        {
            let refer = BON_MGR.clone();
            reply = (&mut *refer.lock().
                                unwrap()).
                                call(js.clone(), hash, vec).
                                ok();
        }
        match reply {
            Some(val) => val.get_value() as *const c_void,
            None => {
                //没有立即返回，则表示会阻塞，并异步返回
                unsafe {
                    if njsc_vm_status_switch(vm, JSStatus::SingleTask as i8, JSStatus::WaitBlock as i8) == JSStatus::SingleTask as i8 {
                        //改变状态成功，防止虚拟机在当前同步任务完成后被立即回收，回收权利交由异步任务
                        null()
                    } else {
                        panic!("native object function call failed");
                    }
                }
            },
        }
}

//转换参数
fn args_to_vec(vm: *const c_void, args_size: u32, args_type: *const u8, args: *const u64) -> Vec<JSType> {
    let mut vec = Vec::new();
    if args_size == 0 {
        return vec;
    }
    
    let mut type_id: u8;
    let mut arg: u64;
    for offset in 0..args_size {
        unsafe {
            type_id = args_type.wrapping_offset(offset as isize).read();
            if type_id <= JSValueType::String as u8 || 
                type_id == JSValueType::ArrayBuffer as u8 ||
                type_id == JSValueType::Uint8Array as u8 ||
                type_id == JSValueType::NativeObject as u8 ||
                type_id == JSValueType::Object as u8 ||
                type_id == JSValueType::Array as u8 {
                    arg = args.wrapping_offset(offset as isize).read();
                    vec.insert(offset as usize, JSType::new(type_id, vm, arg as *const c_void));
            } else {
                panic!("invali native object call arg type, {}", type_id);
            }
        }
    }
    vec
}

//释放指定虚拟机对应的NativeObject实例
#[no_mangle]
pub extern "C" fn native_object_function_free(ptr: *const c_void, size: uint32_t) {
    let mut vec = Vec::with_capacity(size as usize);
    let instances = ptr as *const u64;
    for offset in 0..size {
        vec.insert(offset as usize, unsafe { instances.wrapping_offset(offset as isize).read() });
    }
    //TODO 调用实际的free函数...
}