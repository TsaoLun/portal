use dioxus::prelude::*;
use dioxus_router::RouterService;
use gloo::dialogs::alert;
use std::rc::Rc;

use crate::{
    api::{data, request},
    utils::{api_response::SERVER_ERROR, str_tools::portal},
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
            Err(_) => {
                alert(SERVER_ERROR); // unexpect error
            }

            Ok(data) => match data.data {
                Some(e) => {
                    portal(e.clone());
                    copied_data.set(e);
                }
                None => match data.err {
                    Some(e) => match e.code {
                        Some(inner) => match inner.as_str() {
                            "INVALID_TOKEN" | "EXPIRED_TOKEN" => {
                                router.push_route("/login", None, None)
                            }
                            _ => alert(&e.message),
                        },
                        None => alert(&e.message),
                    },
                    None => {
                        alert(SERVER_ERROR);
                    }
                },
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
            Ok(res) => match res {
                None => state.set("在任意终端按 C 复制".into()),
                Some(e) => {
                    if let Some(code) = e.code {
                        match code.as_str() {
                            "INVALID_TOKEN" | "EXPIRED_TOKEN" => {
                                alert(&e.message);
                                router.push_route("/login", None, None)
                            }
                            _ => alert(&e.message),
                        }
                    } else {
                        alert(&e.message)
                    }
                }
            },
            Err(_) => {
                alert(SERVER_ERROR); // unexpect error
            }
        }
    });
}
pub async fn first_cache(init_data: UseState<String>, router: Rc<RouterService>) {
    let data = data::get_query(request()).await;
    match data {
        Ok(data) => {
            if let Some(err) = data.err {
                match err.code {
                    Some(code) => {
                        alert(&err.message);
                        if (code.as_str() == "INVALID_TOKEN") || (code.as_str() == "EXPIRED_TOKEN")
                        {
                            router.push_route("/login", None, None)
                        }
                    }
                    None => alert(&err.message),
                }
            } else if let Some(data) = data.data {
                init_data.set(data);
            }
        }
        Err(_) => {
            alert(SERVER_ERROR);
            //router.push_route("/login", None, None);
        }
    }
}
