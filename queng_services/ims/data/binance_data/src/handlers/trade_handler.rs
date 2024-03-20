use binance::ws_model::WebsocketEvent;
use tokio::sync::mpsc::UnboundedSender;

pub(crate) fn trade_event_handler(
    event: WebsocketEvent,
    logger_tx: UnboundedSender<WebsocketEvent>,
) {
    match event {
        WebsocketEvent::Trade(ref trade) => {
            println!(
                "Trade: Symbol: {}, price: {}, qty: {}",
                trade.symbol, trade.price, trade.qty
            );
            logger_tx.send(event.clone()).unwrap();
        }

        WebsocketEvent::AggTrade(ref agg_trade) => {
            println!(
                "AggTrade: Symbol: {}, price: {}, qty: {}",
                agg_trade.symbol, agg_trade.price, agg_trade.qty
            );
            logger_tx.send(event.clone()).unwrap();
        }
        _ => {
            println!("Unhandled event: {:?}", event);
        }
    };
}
