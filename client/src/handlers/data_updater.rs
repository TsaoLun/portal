use dioxus::prelude::*;
use dioxus_router::RouterService;
use gloo::dialogs::alert;
use std::rc::Rc;

use crate::{
    api::{data, request},
    utils::{
        api_response::{AppError, SERVER_ERROR},
        str_tools::portal,
    },
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
                portal(data.clone());
                copied_data.set(data);
            }
            Err(e) => match e {
                AppError::NetworkError(_) => alert(SERVER_ERROR),
                AppError::SpecError(e) => match e.as_str() {
                    "EXPIRED_TOKEN" | "INVALID_TOKEN" => router.push_route("/login", None, None),
                    _ => alert(&e),
                },
                AppError::AnyError(e) => alert(&e),
            },
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
            Ok(r) => {
                if r {
                    state.set("在任意终端按 C 复制".into())
                } else {
                    state.set("返回数据异常".into())
                }
            }
            Err(e) => match e {
                AppError::NetworkError(_) => alert(SERVER_ERROR),
                AppError::SpecError(e) => match e.as_str() {
                    "EXPIRED_TOKEN" | "INVALID_TOKEN" => router.push_route("/login", None, None),
                    _ => alert(&e),
                },
                AppError::AnyError(e) => alert(&e),
            },
        }
    });
}
pub async fn first_cache(init_data: UseState<String>, router: Rc<RouterService>) {
    let data = data::get_query(request()).await;
    match data {
        Ok(data) => {
            init_data.set(data);
        }
        Err(err) => match err {
            AppError::NetworkError(_) => alert(SERVER_ERROR),
            AppError::SpecError(e) => match e.as_str() {
                "EXPIRED_TOKEN" | "INVALID_TOKEN" => router.push_route("/login", None, None),
                _ => alert(&e),
            },
            AppError::AnyError(e) => alert(&e),
        },
    }
}
