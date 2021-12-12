use mogwai::prelude::*;
use wasm_bindgen::prelude::*;

pub mod app;
mod components;
mod pages;
pub mod router;
mod util;

use router::{Out, Route};

pub fn styles() -> String {
    let mut styles: Vec<String> = vec![];
    pages::home::styles(&mut styles);
    styles.join("\n")
}

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
                router::push_state(route);
            }
        }
    }

    fn view(&self, tx: broadcast::Sender<Route>, rx: broadcast::Receiver<Out>) -> ViewBuilder<Dom> {
        components::layout::view(tx, rx, ViewBuilder::from(&self.current_route))
    }
}

// use mogwai_hydrator::Hydrator;

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
