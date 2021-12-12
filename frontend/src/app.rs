use crate::{components, pages};
use mogwai::prelude::*;

pub fn styles() -> String {
    let mut styles: Vec<String> = vec![];

    pages::home::styles(&mut styles);

    styles.join("\n")
}

#[derive(Copy, Clone, Debug)]
pub enum Out {
    Render { route: Route },
}

// impl Out {
//     fn maybe_patch_route(&self) -> Option<ListPatch<ViewBuilder<Dom>>> {
//         match self {
//             Out::Render { route } => Some(ListPatch::replace(0, route.into())),
//         }
//     }
// }

#[derive(Debug)]
pub struct App {
    current_route: Route,
}

impl App {
    pub fn component(initial_route: Route) -> Component<Dom> {
        let app = App {
            current_route: initial_route,
        };
        let (tx_logic, rx_logic) = broadcast::bounded(1);
        let (tx_view, rx_view) = broadcast::bounded(1);
        Component::from(app.view(tx_logic, rx_view)).with_logic(app.into_logic(rx_logic, tx_view))
    }

    async fn into_logic(
        mut self,
        mut rx_logic: broadcast::Receiver<Route>,
        tx_view: broadcast::Sender<Out>,
    ) {
        while let Some(route) = rx_logic.next().await {
            if self.current_route != route {
                self.current_route = route;
                tx_view
                    .broadcast(Out::Render {
                        route: self.current_route,
                    })
                    .await
                    .unwrap();
                route_dispatch::push_state(route);
            }
        }
    }

    fn view(&self, tx: broadcast::Sender<Route>, rx: broadcast::Receiver<Out>) -> ViewBuilder<Dom> {
        components::layout::view(tx, rx, ViewBuilder::from(&self.current_route))
    }
}

// use mogwai_hydrator::Hydrator;
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Route {
    Home,
    Dashboard,
    Projects,
    Staking,
    Organization,
    NotFound,
}

#[cfg(not(target_arch = "wasm32"))]
/// On the server we create our app's view using the an initial route
/// and then we stringify the view.
pub fn view<T>(path: T) -> Result<String, String>
where
    T: AsRef<str>,
{
    let initial_route: Route = path.into();
    let view: View<Dom> = App::component(initial_route).build()?;
    Ok(String::from(view))
}

pub fn new() -> Result<(), JsValue> {
    let initial_route = Route::from(utils::window().location().pathname().unwrap_throw());
    let root: Component<Dom> = App::component(initial_route);
    // let hydrator = Hydrator::try_from(root).map_err(|e| JsValue::from(format!("{}", e)))?;
    // let view = View::from(hydrator);
    let view = root.build().unwrap();
    view.run()
}

/// Dispatches the given `Route`.
pub mod route_dispatch {
    use crate::app::Route;
    use wasm_bindgen::prelude::JsValue;

    pub fn push_state(route: Route) {
        let window = mogwai::utils::window();
        match window.history() {
            Ok(history) => {
                let state = JsValue::from("");
                let push_result =
                    history.push_state_with_url(&state, "", Some(&format!("{}", route)));
                if let Err(error) = push_result {
                    ::log::debug!("{:?}", error);
                }
            }
            Err(error) => ::log::debug!("{:?}", error),
        }
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
