use anyhow::{anyhow, Result};
use libc::c_void;

use open62541_ffi as open62541;
use open62541::{UA_Server, UA_NodeId, UA_NumericRange, UA_DataValue, UA_StatusCode};

use signal_hook::iterator::Signals;
use std::{
    sync::{atomic::AtomicBool, atomic::AtomicI32, Arc, atomic::Ordering},
    thread,
    time::Duration
};

#[macro_use]
extern crate lazy_static;

lazy_static!{
    static ref MY_INT : Arc<AtomicI32> = Arc::new(AtomicI32::new(0 as i32));
}

#[no_mangle]
pub unsafe extern "C" fn read_data_source (_server: *mut UA_Server,
        _session_id: *const UA_NodeId, _session_context: *mut c_void,
        _node_id: *const UA_NodeId, _node_context: *mut c_void,
        _source_timestamp: bool, _range: *const UA_NumericRange, data_value: *mut UA_DataValue)
        -> UA_StatusCode {
    let my_integer = MY_INT.load(Ordering::Relaxed);
    
    let my_type = &open62541::UA_TYPES[open62541::UA_TYPES_INT32 as usize];

    let retval = 
        open62541::UA_Variant_setScalarCopy(
            &mut (*data_value).value as *mut open62541::UA_Variant,
            &my_integer as *const _ as *const c_void,
            my_type,
        );
    if retval != 0 {
        println!("UA_Variant_setScalarCopy returned {}", retval);
    }

    open62541::UA_STATUSCODE_GOOD
}

pub unsafe extern "C" fn write_data_source (_server: *mut UA_Server,
        _session_id: *const UA_NodeId, _session_context: *mut c_void,
        _node_id: *const UA_NodeId, _node_context: *mut c_void,
        _range: *const UA_NumericRange, _data_value: *const UA_DataValue) -> UA_StatusCode {
    open62541::UA_STATUSCODE_BADINTERNALERROR
}

fn main() -> Result<()> {
    let mut signals = Signals::new(&[signal_hook::consts::SIGINT, signal_hook::consts::SIGTERM])?;

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    thread::spawn(move || {
        if let Some(_) = signals.into_iter().next() {
            running_clone.store(false, std::sync::atomic::Ordering::Relaxed);
        }
    });

    let my_int_clone : Arc<AtomicI32> = MY_INT.clone();
    let r_2 = running.clone();
    thread::spawn(move || {
        while r_2.load(Ordering::SeqCst) {
            my_int_clone.fetch_add(1, Ordering::Relaxed);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    let server = unsafe { open62541::UA_Server_new() };
    let config = unsafe { open62541::UA_Server_getConfig(server) };
    let retval = unsafe {
        open62541::UA_ServerConfig_setMinimalCustomBuffer(config, 4840, std::ptr::null(), 0, 0)
    };
    if retval != 0 {
        return Err(anyhow!(
            "UA_ServerConfig_setMinimalCustomBuffer returned {}",
            retval
        ));
    }

    let running = Arc::<AtomicBool>::as_ptr(&running).cast();

    let mut attr = unsafe { open62541::UA_VariableAttributes_default };
    let my_integer = 42;
    let my_type = unsafe { &open62541::UA_TYPES[open62541::UA_TYPES_INT32 as usize] };   

    let retval = unsafe {
        open62541::UA_Variant_setScalarCopy(
            &mut attr.value as *mut open62541::UA_Variant,
            &my_integer as *const _ as *const c_void,
            my_type,
        )
    };
    if retval != 0 {
        return Err(anyhow!("UA_Variant_setScalarCopy returned {}", retval));
    }

    unsafe {
        attr.description = open62541::UA_LocalizedText {
            locale: open62541::UA_String_fromChars(b"en-US\0".as_ptr() as *const _),
            text: open62541::UA_String_fromChars(b"the answer\0".as_ptr() as *const _),
        };
        attr.displayName = open62541::UA_LocalizedText {
            locale: open62541::UA_String_fromChars(b"en-US\0".as_ptr() as *const _),
            text: open62541::UA_String_fromChars(b"the answer\0".as_ptr() as *const _),
        };
    }

    let my_integer_node_id = open62541::UA_NodeId {
        namespaceIndex: 1,
        identifierType: open62541::UA_NodeIdType_UA_NODEIDTYPE_STRING,
        identifier: open62541::UA_NodeId__bindgen_ty_1 {
            string: unsafe {
                open62541::UA_String_fromChars(b"the.answer\0".as_ptr() as *const _)
            },
        },
    };

    let my_integer_name = open62541::UA_QualifiedName {
        namespaceIndex: 1,
        name: unsafe { open62541::UA_String_fromChars(b"the answer\0".as_ptr() as *const _) },
    };

    let mut attr_2 = unsafe { open62541::UA_VariableAttributes_default };

    unsafe {
        attr_2.description = open62541::UA_LocalizedText {
            locale: open62541::UA_String_fromChars(b"en-US\0".as_ptr() as *const _),
            text: open62541::UA_String_fromChars(b"incrementing integer\0".as_ptr() as *const _),
        };
        attr_2.displayName = open62541::UA_LocalizedText {
            locale: open62541::UA_String_fromChars(b"en-US\0".as_ptr() as *const _),
            text: open62541::UA_String_fromChars(b"incrementing integer\0".as_ptr() as *const _),
        };
    }

    let my_integer_node_id_2 = open62541::UA_NodeId {
        namespaceIndex: 1,
        identifierType: open62541::UA_NodeIdType_UA_NODEIDTYPE_STRING,
        identifier: open62541::UA_NodeId__bindgen_ty_1 {
            string: unsafe {
                open62541::UA_String_fromChars(b"incrementing.integer\0".as_ptr() as *const _)
            },
        },
    };

    let my_integer_name_2 = open62541::UA_QualifiedName {
        namespaceIndex: 1,
        name: unsafe { open62541::UA_String_fromChars(b"incrementing integer\0".as_ptr() as *const _) },
    };

    let parent_node_id = open62541::UA_NodeId {
        namespaceIndex: 0,
        identifierType: open62541::UA_NodeIdType_UA_NODEIDTYPE_NUMERIC,
        identifier: open62541::UA_NodeId__bindgen_ty_1 {
            numeric: open62541::UA_NS0ID_OBJECTSFOLDER,
        },
    };

    let parent_reference_node_id = open62541::UA_NodeId {
        namespaceIndex: 0,
        identifierType: open62541::UA_NodeIdType_UA_NODEIDTYPE_NUMERIC,
        identifier: open62541::UA_NodeId__bindgen_ty_1 {
            numeric: open62541::UA_NS0ID_ORGANIZES,
        },
    };

    let retval = unsafe {
        open62541::__UA_Server_addNode(
            server,
            open62541::UA_NodeClass_UA_NODECLASS_VARIABLE,
            &my_integer_node_id as *const open62541::UA_NodeId,
            &parent_node_id as *const open62541::UA_NodeId,
            &parent_reference_node_id as *const open62541::UA_NodeId,
            my_integer_name,
            &open62541::UA_NODEID_NULL as *const open62541::UA_NodeId,
            std::mem::transmute(&attr as *const open62541::UA_VariableAttributes),
            &open62541::UA_TYPES[open62541::UA_TYPES_VARIABLEATTRIBUTES as usize],
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };

    if retval != 0 {
        return Err(anyhow!("__UA_Server_addNode returned {}", retval));
    }

    let variable_type_node_id = open62541::UA_NodeId {
        namespaceIndex: 0,
        identifierType: open62541::UA_NodeIdType_UA_NODEIDTYPE_NUMERIC,
        identifier: open62541::UA_NodeId__bindgen_ty_1 {
            numeric: open62541::UA_NS0ID_BASEDATAVARIABLETYPE,
        }
    };

    let my_data_source = open62541::UA_DataSource{
        read : Some(read_data_source),
        write : Some(write_data_source),
    };

    let retval_2 = unsafe {
        open62541::UA_Server_addDataSourceVariableNode(
            server,
            my_integer_node_id_2 as open62541::UA_NodeId,
            parent_node_id as open62541::UA_NodeId,
            parent_reference_node_id as open62541::UA_NodeId,
            my_integer_name_2,
            variable_type_node_id as open62541::UA_NodeId,
            attr_2 as open62541::UA_VariableAttributes,
            my_data_source,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
    )};

    if retval_2 != 0 {
        println!("retval_2 returned {}", retval_2);
    }

    unsafe {
        open62541::UA_clear(
            &attr as *const _ as *mut c_void,
            &open62541::UA_TYPES[open62541::UA_TYPES_VARIABLEATTRIBUTES as usize],
        );
        open62541::UA_clear(
            &my_integer_node_id as *const _ as *mut c_void,
            &open62541::UA_TYPES[open62541::UA_TYPES_NODEID as usize],
        );
        open62541::UA_clear(
            &my_integer_name as *const _ as *mut c_void,
            &open62541::UA_TYPES[open62541::UA_TYPES_QUALIFIEDNAME as usize],
        );
        open62541::UA_clear(
            &my_integer_node_id_2 as *const _ as *mut c_void,
            &open62541::UA_TYPES[open62541::UA_TYPES_NODEID as usize],
        );
        open62541::UA_clear(
            &my_integer_name_2 as *const _ as *mut c_void,
            &open62541::UA_TYPES[open62541::UA_TYPES_QUALIFIEDNAME as usize],
        );
    }

    let retval = unsafe { open62541::UA_Server_run(server, running) };

    if retval != 0 {
        return Err(anyhow!("UA_Server_run returned {}", retval));
    }

    unsafe { open62541::UA_Server_delete(server) };

    Ok(())
}
