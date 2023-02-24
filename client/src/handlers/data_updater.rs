use dioxus::prelude::*;
use dioxus_router::RouterService;
use gloo::dialogs::alert;
use std::rc::Rc;

use crate::{
    api::{data, request},
    utils::str_tools::portal,
};
pub fn copy_data(
    cx: Scope,
    (copied_data, init_data): (UseState<String>, UseState<String>),
    router: Rc<RouterService>,
) {
    if copied_data.get() == "" {
        portal(init_data.get().into());
    } else {
        portal(copied_data.get().into());
    }
    cx.spawn(async move {
        let data = data::get_query(request()).await;
        match data {
            Ok(data) => {
                portal(data.to_string());
                copied_data.set(data);
            }
            Err(_) => {
                alert("登录过期，请重新登录");
                router.push_route("/login", None, None);
            }
        }
    });
}
pub fn submit_data(
    cx: Scope,
    (state, data): (UseState<String>, String),
    router: Rc<RouterService>,
) {
    cx.spawn(async move {
        let res = data::set_mutation(request(), data).await;
        match res {
            Ok(_data) => {
                state.set("在任意终端按 C 复制".into());
            }
            Err(_) => {
                alert("登录过期，请重新登录");
                router.push_route("/login", None, None);
            }
        }
    });
}
pub async fn first_cache(init_data: UseState<String>, router: Rc<RouterService>) {
    let data = data::get_query(request()).await;
    match data {
        Ok(data) => {
            init_data.set(data);
        }
        Err(_) => {
            alert("登录过期，请重新登录");
            router.push_route("/login", None, None);
        }
    }
}
