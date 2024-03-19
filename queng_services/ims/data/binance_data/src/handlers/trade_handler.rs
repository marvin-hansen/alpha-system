use binance::ws_model::WebsocketEvent;

pub(crate) fn trade_event_handler(event: WebsocketEvent) {
    match event {
        WebsocketEvent::Trade(trade) => {
            println!(
                "Symbol: {}, price: {}, qty: {}",
                trade.symbol, trade.price, trade.qty
            );
        }
        _ => (),
    };
}
