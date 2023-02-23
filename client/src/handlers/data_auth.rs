use dioxus::prelude::*;
use dioxus_router::RouterService;
use gloo::dialogs::alert;
use std::rc::Rc;

use crate::api::{login, request};

pub fn submit_login(cx: Scope, (usernmae, password): (String, String), router: Rc<RouterService>) {
    cx.spawn(async move {
        let resp = login::login(request(), usernmae, password).await;

        match resp {
            // Parse data from here, such as storing a response token
            Ok(ok) => {
                if ok {
                    router.push_route("/", None, None);
                } else {
                    alert("账号密码有误");
                }
            }
            // Handle any errors from the fetch here
            Err(_err) => {
                alert("服务器异常");
            }
        }
    });
}
