use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

use quinn::{ClientConfig, Endpoint, Incoming, ServerConfig};
use std::{error::Error, net::SocketAddr, sync::Arc};

#[no_mangle]
pub extern "system" fn Java_com_crylog_quinn4android_Quinn4Android_Client_1Insecure(
    env: JNIEnv, _class: JClass, ip: JString) -> jstring {
    let ip_rs: String = env.get_string(ip).expect("Couldn't get java string!").into();
    let output = env.new_string(format!("Client say hello : {}!", ip_rs)).expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_crylog_quinn4android_Quinn4Android_Server_1Insecure(
    env: JNIEnv, _class: JClass, ip: JString) -> jstring {
    let ip_rs: String = env.get_string(ip).expect("Couldn't get java string!").into();
    let output = env.new_string(format!("Server say hello : {}!", ip_rs)).expect("Couldn't create java string!");
    output.into_inner()
}


