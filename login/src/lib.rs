use actix_web::Responder;
use plugin_lib::{declare_plugin, register_plugin};

#[derive(Debug)]
struct LoginPlugin;
impl LoginPlugin {
    pub fn new() -> Self {
        Self
    }
    async fn login() -> impl Responder {
        "Login Page"
    }
}

declare_plugin!(LoginPlugin, "LoginPlugin","0.1.0", "Login Plugin","/auth",functions:{
    "/login" => {
        method: actix_web::web::get(),
        handler: LoginPlugin::login
    }
});

register_plugin!(LoginPlugin);
