use futures::{SinkExt, StreamExt};
use mogwai::prelude::*;
use stylist::style;

#[cfg(target_arch = "wasm32")]
use log::info;
#[cfg(target_arch = "wasm32")]
use web3::{
    transports::eip_1193::{Eip1193, Provider},
    Web3,
};

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
            mogwai::spawn(async move {
                tx_view.broadcast(State::Connecting).await.unwrap();
                // web3::block_on(connect_web3()).unwrap();
                connect_web3().await;
                tx_view.broadcast(State::Connected).await.unwrap();
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

pub async fn connect_web3() {
    #[cfg(target_arch = "wasm32")]
    if let Some(provider) = Provider::default().unwrap() {
        let transport = Eip1193::new(provider);
        let web3 = Web3::new(transport);
        info!("requesting...");
        // let accounts = web3::block_on(web3.eth().request_accounts())?;
        let accounts = web3.eth().request_accounts().await.unwrap();
        info!("accounts: {:?}", accounts);
    }
}

pub fn new() -> Component<Dom> {
    ConnectButton {
        status: State::Connect,
    }
    .to_component()
}
