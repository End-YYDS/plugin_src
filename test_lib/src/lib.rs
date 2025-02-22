use actix_web::Responder;
use plugin_lib::{declare_plugin, register_plugin};

#[derive(Debug)]
#[allow(non_camel_case_types)]
struct test_lib_Plugin;

impl test_lib_Plugin {
    pub fn new() -> Self {
        Self
    }

    async fn test() -> impl Responder {
        "test lib"
    }
}

declare_plugin!(
    test_lib_Plugin,
    meta: {"test_lib_Plugin","0.1.0", "test lib","/test",""},
    "test_lib.js",
    functions:{
        "/test" => {
            method: actix_web::web::get(),
            handler: test_lib_Plugin::test
        }
    }
);

register_plugin!(test_lib_Plugin);