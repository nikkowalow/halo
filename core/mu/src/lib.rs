pub mod matching;

use colored::*;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_postgres::Error;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum ClientMessage {
    BuyTicket {
        #[serde(rename = "eventId")]
        event_id: i32,
        qty: i64,
    },
}
#[tokio::main]
pub async fn run() -> Result<(), Error> {
    println!("{} {} {}", "starting", "MU".red().bold(), "...");
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Failed to accept");
            println!("New WebSocket connection!");

            let (mut write, mut read) = ws_stream.split();

            write
                .send(Message::Text(String::from(
                    "Welcome to the WebSocket server!",
                )))
                .await
                .expect("Failed to send message");

            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => match msg {
                        Message::Text(text) => {
                            println!("Received text message: {}", text);

                            match serde_json::from_str::<ClientMessage>(&text) {
                                Ok(ClientMessage::BuyTicket { event_id, qty }) => {
                                    let ticket = matching::BuyTicket {
                                        event_id,
                                        amount: qty,
                                    };
                                    let result = matching::buy_ticket(ticket).await;
                                    write
                                        .send(Message::Text(
                                            result.unwrap_or("error buying ticket".to_string()),
                                        ))
                                        .await
                                        .expect("Failed to send message");
                                }
                                Err(e) => {
                                    println!("Failed to parse JSON: {}", e);
                                    write
                                        .send(Message::Text("invalid message format".to_string()))
                                        .await
                                        .expect("Failed to send message");
                                }
                            }
                        }
                        Message::Binary(_) => {
                            println!("Binary message received – ignoring.");
                        }
                        Message::Ping(ping) => {
                            println!("Ping received – responding with Pong.");
                            write.send(Message::Pong(ping)).await.ok();
                        }
                        Message::Pong(_) => {
                            println!("Pong received.");
                        }
                        Message::Close(frame) => {
                            println!("Connection closed: {:?}", frame);
                        }
                        _ => {
                            println!("Unhandled message type");
                        }
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }
        });
    }
    Ok(())
}
