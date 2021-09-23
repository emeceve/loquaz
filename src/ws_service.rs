use druid::{ExtEventSink, Target};
use futures::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::delegate::WS_RECEIVED_DATA;

pub async fn connect(
    event_sink: ExtEventSink,
    ws_url: String,
    msg_rx: futures_channel::mpsc::UnboundedReceiver<Message>,
) {
    let url = url::Url::parse(&ws_url).unwrap();

    println!("Trying to connect...");
    let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect!");
    println!("Successfully connected to relay {}!", &url.to_string());

    let (ws_tx, mut ws_rx) = ws_stream.split();

    let from_msg_to_ws = msg_rx.map(Ok).forward(ws_tx);

    tokio::spawn(from_msg_to_ws);
    //    let read_stream = {
    //        read.for_each(|msg| async {
    //            let data = msg.unwrap().into_data();
    //            event_sink
    //                .submit_command(
    //                    SET_COUNTER,
    //                    String::from(std::str::from_utf8(data.as_ref()).unwrap()),
    //                    Target::Auto,
    //                )
    //                .expect("Error");
    //            tokio::io::stdout().write_all(&data).await.unwrap();
    //        })
    //    };
    while let Some(msg) = ws_rx.next().await {
        let data = msg.unwrap().into_data();
        //let data_to_str = std::str::from_utf8(data.as_ref()).unwrap();
        let data_to_str = String::from_utf8(data).unwrap();
        println!("[Received data]: {}", &data_to_str);
        event_sink
            .submit_command(WS_RECEIVED_DATA, data_to_str, Target::Auto)
            .expect("Error");
    }
}
