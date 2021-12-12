use mogwai::prelude::*;
use wasm_bindgen::prelude::*;

use crate::pages;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Route {
    Home,
    Dashboard,
    Projects,
    Staking,
    Organization,
    NotFound,
}

#[derive(Copy, Clone, Debug)]
pub enum Out {
    Render { route: Route },
}

impl Out {
    pub fn maybe_patch_route(&self) -> Option<ListPatch<ViewBuilder<Dom>>> {
        match self {
            Out::Render { route } => Some(ListPatch::replace(0, route.into())),
        }
    }
}

pub fn push_state(route: Route) {
    let window = mogwai::utils::window();
    match window.history() {
        Ok(history) => {
            let state = JsValue::from("");
            let push_result = history.push_state_with_url(&state, "", Some(&format!("{}", route)));
            if let Err(error) = push_result {
                ::log::debug!("{:?}", error);
            }
        }
        Err(error) => ::log::debug!("{:?}", error),
    }
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Route::Home => f.write_str("/"),
            Route::Dashboard => f.write_str("/dashboard"),
            Route::Projects => f.write_str("/projects"),
            Route::Staking => f.write_str("/staking"),
            Route::Organization => f.write_str("/organization"),
            Route::NotFound => f.write_str("/404"),
        }
    }
}

impl<T: AsRef<str>> From<T> for Route {
    fn from(path: T) -> Self {
        let s = path.as_ref();
        // remove the scheme, if it has one
        let paths: Vec<&str> = s.split('/').collect::<Vec<_>>();

        match paths.as_slice() {
            [""] => Route::Home,
            ["", ""] => Route::Home,
            ["", "dashboard"] => Route::Dashboard,
            ["", "projects"] => Route::Projects,
            ["", "staking"] => Route::Staking,
            ["", "organization"] => Route::Organization,
            _ => Route::NotFound,
        }
    }
}

impl From<&Route> for ViewBuilder<Dom> {
    fn from(route: &Route) -> ViewBuilder<Dom> {
        match route {
            Route::Home => pages::home::view(),
            Route::Dashboard => pages::dashboard::view(),
            Route::Projects => pages::projects::view(),
            Route::Staking => pages::staking::view(),
            Route::Organization => pages::organization::view(),
            Route::NotFound => pages::not_found::view(),
        }
    }
}
