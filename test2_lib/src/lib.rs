use actix_web::Responder;
use plugin_lib::{declare_plugin, register_plugin};

#[derive(Debug)]
#[allow(non_camel_case_types)]
struct test2_lib_Plugin;

impl test2_lib_Plugin {
    pub fn new() -> Self {
        Self
    }

    async fn test() -> impl Responder {
        "test lib"
    }
}

declare_plugin!(
    test2_lib_Plugin,
    meta: {"test2_lib_Plugin","0.1.0", "test lib","/test","5b38abff91682182a95480a9f98eec477bba72e641cf38b8a12e313311435d0a"},
    "test2_lib.js",
    functions:{
        "/test" => {
            method: actix_web::web::get(),
            handler: test2_lib_Plugin::test
        }
    }
);

register_plugin!(test2_lib_Plugin);
