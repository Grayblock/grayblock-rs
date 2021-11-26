#![allow(unused_braces)]
use mogwai::prelude::*;

#[cfg(test)]
mod test {
    use mogwai::prelude::*;

    #[test]
    fn can_component_from_viewbuilder() {
        let _comp = Component::from(builder! {
            <div id="my_component">
                <p>"Hello!"</p>
            </div>
        });
    }

    #[test]
    fn can_component_logic() {
        let (tx, rx) = broadcast::bounded::<u32>(1);
        let comp = Component::from(builder! {
            <div id="my_component">
                <p>
                    {("initial value", rx.map(|n| format!("got message: {}", n)))}
                </p>
            </div>
        })
        .with_logic(async move {
            tx.broadcast(1).await.unwrap();
            tx.broadcast(42).await.unwrap();
        });
        let view: View<Dom> = comp.build().unwrap();
        view.run().unwrap();
    }

    #[test]
    fn can_more_component_logic() {
        let (tx_logic, mut rx_logic) = broadcast::bounded::<()>(1);
        let (tx_view, rx_view) = broadcast::bounded::<u32>(1);

        let comp = Component::from(builder! {
            <div id="my_component" on:click=tx_logic.sink().contra_map(|_| ())>
                <p>
                    {("initial value", rx_view.map(|n| format!("got clicks: {}", n)))}
                </p>
            </div>
        })
        .with_logic(async move {
            let mut clicks = 0;
            tx_view.broadcast(clicks).await.unwrap();

            loop {
                match rx_logic.next().await {
                    Some(()) => {
                        clicks += 1;
                        tx_view.broadcast(clicks).await.unwrap();
                    }
                    None => break,
                }
            }
        });
        let view: View<Dom> = comp.build().unwrap();
        view.run().unwrap();
    }
}

mod counter {
    use mogwai::prelude::{stream::select_all, *};

    #[derive(Clone)]
    pub enum CounterMsg {
        Click,
        Reset,
    }

    fn view(
        send_clicks_to_logic: broadcast::Sender<CounterMsg>,
        recv_num_clicks: broadcast::Receiver<u32>,
    ) -> ViewBuilder<Dom> {
        builder! {
            <button on:click=send_clicks_to_logic.sink().with(|_| async{Ok(CounterMsg::Click)})>
            {(
                "clicks = 0",
                recv_num_clicks.map(|n| format!("clicks = {}", n))
            )}
            </button>
        }
    }

    async fn logic(
        mut recv_msg: impl Stream<Item = CounterMsg> + Unpin,
        send_num_clicks: broadcast::Sender<u32>,
    ) {
        let mut clicks: u32 = 0;
        loop {
            match recv_msg.next().await {
                Some(CounterMsg::Click) => {
                    clicks += 1;
                }
                Some(CounterMsg::Reset) => {
                    clicks = 0;
                }
                None => break,
            }

            send_num_clicks.broadcast(clicks).await.unwrap();
        }
    }

    pub fn counter(recv_parent_msg: broadcast::Receiver<CounterMsg>) -> Component<Dom> {
        let (send_self_msg, recv_self_msg) = broadcast::bounded(1);
        let (send_num_clicks, recv_num_clicks) = broadcast::bounded(1);
        let counter_view = view(send_self_msg, recv_num_clicks);
        let counter_logic = logic(
            select_all(vec![recv_self_msg, recv_parent_msg]),
            send_num_clicks,
        );
        Component::from(counter_view).with_logic(counter_logic)
    }
}

fn view(counter: Component<Dom>, send_reset_to_app: broadcast::Sender<()>) -> ViewBuilder<Dom> {
    builder! {
        <div>
            "Application"<br/>
            {counter}
            <button on:click=send_reset_to_app.sink().with(|_| async{Ok(())})>"Click to reset"</button>
        </div>
    }
}

async fn logic(
    send_reset_to_counter: broadcast::Sender<counter::CounterMsg>,
    mut recv_reset: broadcast::Receiver<()>,
) {
    loop {
        match recv_reset.next().await {
            Some(()) => {
                send_reset_to_counter
                    .broadcast(counter::CounterMsg::Reset)
                    .await
                    .unwrap();
            }
            None => break,
        }
    }
}

pub fn new() -> Component<Dom> {
    let (send_counter_msg, recv_counter_msg) = broadcast::bounded(1);
    let (send_reset_to_app, recv_reset_from_app) = broadcast::bounded(1);

    let app_logic = logic(send_counter_msg.clone(), recv_reset_from_app);
    let counter = counter::counter(recv_counter_msg);
    let app_view = view(counter, send_reset_to_app);
    Component::from(app_view).with_logic(app_logic)
}
