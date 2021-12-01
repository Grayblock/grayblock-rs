#![allow(unused_braces)]
use log::{trace, Level};
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::HashChangeEvent;

/// Here we enumerate all our app's routes.
#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Home,
    About,
    Dashboard,
    Offerings,
    Projects,
}

/// We'll use TryFrom::try_from to convert the window's url hash into a Route.
impl TryFrom<&str> for Route {
    type Error = String;

    fn try_from(s: &str) -> Result<Route, String> {
        trace!("route try_from: {}", s);
        // remove the scheme, if it has one
        let hash_split = s.split('#').collect::<Vec<_>>();
        let after_hash = match hash_split.as_slice() {
            [_, after] => Ok(after),
            _ => Err(format!("route must have a hash: {}", s)),
        }?;

        let paths: Vec<&str> = after_hash.split('/').collect::<Vec<_>>();
        trace!("route paths: {:?}", paths);

        match paths.as_slice() {
            [""] => Ok(Route::Home),
            ["", ""] => Ok(Route::Home),
            ["", "about"] => Ok(Route::About),
            ["", "dashboard"] => Ok(Route::Dashboard),
            ["", "offerings"] => Ok(Route::Offerings),
            ["", "projects"] => Ok(Route::Projects),
            r => Err(format!("unsupported route: {:?}", r)),
        }
    }
}

#[cfg(test)]
mod test_route_try_from {
    use super::*;

    #[test]
    fn can_convert_string_to_route() {
        let s = "https://localhost:8080/#/";
        assert_eq!(Route::try_from(s), Ok(Route::Home));
    }
}

/// Convert the route into its hashed string.
/// This should match the inverse conversion in TryFrom above.
impl From<Route> for String {
    fn from(route: Route) -> String {
        match route {
            Route::Home => "#/".into(),
            Route::About => "#/about".into(),
            Route::Dashboard => "#/dashboard".into(),
            Route::Offerings => "#/offerings".into(),
            Route::Projects => "#/projects".into(),
        }
    }
}

/// We can convert a route into a ViewBuilder in order to embed it in a component.
/// This is just a suggestion for this specific example. The general idea is
/// to use the route to inform your app that it needs to change the page. This
/// is just one of many ways to accomplish that.
impl From<&Route> for ViewBuilder<Dom> {
    fn from(route: &Route) -> Self {
        match route {
            Route::Home => builder! {
                <main>
                    <h1>"Grayblock Power"</h1>
                    {grayblock_design::connect_button::new()}
                </main>
            },
            Route::About => builder! {
                <main>
                    <h1>"About Grayblock Power"</h1>
                </main>
            },
            Route::Dashboard => builder! {
                <main>
                    <h1>"Dashboard"</h1>
                </main>
            },
            Route::Offerings => builder! {
                <main>
                    <h1>"Energy Lending Offerings"</h1>
                </main>
            },
            Route::Projects => builder! {
                <main>
                    <h1>"Energy Projects"</h1>
                </main>
            },
        }
    }
}

#[derive(Clone)]
enum AppModel {
    HashChange(String),
}

#[derive(Clone)]
struct AppError(String);

async fn logic(
    mut route: Route,
    mut rx_logic: broadcast::Receiver<AppModel>,
    tx_view: broadcast::Sender<AppError>,
    tx_route_patch: mpmc::Sender<ListPatch<ViewBuilder<Dom>>>,
) {
    while let Some(AppModel::HashChange(hash)) = rx_logic.next().await {
        // When we get a hash change, attempt to convert it into one of our routes
        match Route::try_from(hash.as_str()) {
            // If we can't, let's send an error message to the view
            Err(msg) => {
                tx_view.broadcast(AppError(msg)).await.unwrap();
            }
            // If we _can_, create a new view from the route and send a patch message to
            // the view
            Ok(new_route) => {
                trace!("got new route: {:?}", new_route);
                if new_route != route {
                    let builder = ViewBuilder::from(&new_route);
                    route = new_route;
                    let patch = ListPatch::replace(2, builder);
                    tx_route_patch.send(patch).await.unwrap();
                }
                tx_view.broadcast(AppError("".to_string())).await.unwrap();
            }
        }
    }
}

fn view(
    route: &Route,
    tx_logic: broadcast::Sender<AppModel>,
    rx_view: broadcast::Receiver<AppError>,
    rx_route_patch: mpmc::Receiver<ListPatch<ViewBuilder<Dom>>>,
) -> ViewBuilder<Dom> {
    builder! {
        <slot
            window:hashchange=tx_logic.sink().contra_map(|ev:Event| {
                let hev = ev.dyn_ref::<HashChangeEvent>().unwrap().clone();
                let hash = hev.new_url();
                AppModel::HashChange(hash)
            })
            patch:children=rx_route_patch>
            <nav>
                <ul>
                    <li>
                        <a href=String::from(Route::Home)>"Home"</a>
                    </li>
                    <li>
                        <a href=String::from(Route::About)>"About"</a>
                    </li>
                    <li>
                        <a href=String::from(Route::Dashboard)>"Dashboard"</a>
                    </li>
                    <li>
                        <a href=String::from(Route::Offerings)>"Offerings"</a>
                    </li>
                    <li>
                        <a href=String::from(Route::Projects)>"Projects"</a>
                    </li>
                </ul>
            </nav>
            <pre>{("", rx_view.map(|AppError(msg)| msg))}</pre>
            {route}
        </slot>
    }
}

pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let route = Route::Home;
    let (tx_logic, rx_logic) = broadcast::bounded(1);
    let (tx_view, rx_view) = broadcast::bounded(1);
    let (tx_route_patch, rx_route_patch) = mpmc::bounded(1);
    let component = Component::from(view(&route, tx_logic, rx_view, rx_route_patch))
        .with_logic(logic(route, rx_logic, tx_view, tx_route_patch));
    let view = component.build().unwrap();

    view.run()
}
