use binance::websockets::WebsocketEvent;

pub(crate) fn handle(event: WebsocketEvent) -> binance::errors::Result<()> {
    match event {
        WebsocketEvent::Trade(trade_event) => {
            println!("{:?}", trade_event);
        }
        _ => (),
    };

    Ok(())
}
