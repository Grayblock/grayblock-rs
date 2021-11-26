use mogwai::prelude::*;
use stylist::style;

pub struct ConnectButton {
    pub status: State,
}

#[derive(Clone)]
pub enum State {
    Connect,
    Connecting,
    Connected,
    Error,
}

impl IsElmComponent for ConnectButton {
    type LogicMsg = State;
    type ViewMsg = State;
    type ViewNode = Dom;

    fn update(&mut self, msg: State, tx_view: broadcast::Sender<State>) {
        if let State::Connect = msg {
            self.status = State::Connecting;
            let out = State::Connecting;
            mogwai::spawn(async move {
                // TODO: web3
                tx_view.broadcast(out).await.unwrap();
            });
        }
    }

    fn view(
        &self,
        tx: broadcast::Sender<State>,
        rx: broadcast::Receiver<State>,
    ) -> ViewBuilder<Dom> {
        let styles = style!(
            r#"
                background-color: #1fc7d4;
                color: #fff;
                width: 100%;
                padding: 24px;
                height: 48px;
                font-size: 16px;
                font-weight: 600;
                display: inline-flex;
                align-items: center;
                justify-content: center;
                border-radius: 16px;
                border: none;
                outline: none;
                cursor: pointer;
            "#
        )
        .unwrap();

        builder! {
            <button class={styles.get_class_name()} on:click=tx.sink().with(|_| async {Ok(State::Connect)})>
                {(
                    map_status(&self.status),
                    rx.map(|msg| map_status(&msg))
                )}
            </button>
        }
    }
}

fn map_status(status: &State) -> String {
    match status {
        State::Connect => "Connect",
        State::Connecting => "Connecting...",
        State::Connected => "Connected",
        State::Error => "Error",
    }
    .to_string()
}

pub fn new() -> Component<Dom> {
    ConnectButton {
        status: State::Connect,
    }
    .to_component()
}
