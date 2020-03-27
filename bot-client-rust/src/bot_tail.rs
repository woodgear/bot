use std::pin::Pin;

use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::stream::*;
use async_std::task::{Context, Poll};
use async_tungstenite::{async_std::connect_async, tungstenite::Message, WebSocketStream};
use bot::protocol::*;

type WsStream = WebSocketStream<TcpStream>;
pub struct BotTail {
    pub server: String,
    pub path: String,
    pub ws_stream: Pin<Box<WsStream>>,
}

impl BotTail {
    pub async fn new(server: &str, path: &str) -> Result<Self, failure::Error> {
        use futures::sink::SinkExt;
        let (mut ws_stream, _) = connect_async(server).await?;

        let bin_msg = ServerAst::Tail(path.to_string()).into_binary()?;

        ws_stream.send(Message::Binary(bin_msg)).await?;
        Ok(Self {
            server: server.to_string(),
            path: path.to_string(),
            ws_stream: Box::pin(ws_stream),
        })
    }
}

use async_std::stream::Stream;
impl Stream for BotTail {
    // we will be counting with usize
    type Item = String;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        println!("{:?}", "poll_next");
        // use async_std::stream::Stream;
        // self.ws_stream.map(|e|"".to_string());
        let msg_bin = if let Some(Ok(Message::Binary(bin))) =
            futures::ready!(self.ws_stream.as_mut().poll_next(cx))
        {
            bin
        } else {
            println!("{:?}", "other");
            return Poll::Ready(None);
        };
        println!("{:?}", "bin");
        match ServerResponse::from_binary(&msg_bin) {
            Ok(ServerResponse::TailResult(TailResult::Err(e))) => {
                println!("tail err {:?}", e);
                return Poll::Ready(None);
            }
            Ok(ServerResponse::TailResult(TailResult::TailContinue(line))) => {
                println!("some l {}", line);
                return Poll::Ready(Some(line));
            }
            Ok(ServerResponse::TailResult(TailResult::TailTimeout)) => {
                println!("tail timeout");
                return Poll::Pending;
            }
            Ok(ServerResponse::TailResult(TailResult::TailEnd)) => {
                println!("tail end");
                return Poll::Ready(None);
            }
            Ok(_) => {
                println!("tail other");
                return Poll::Ready(None);
            }
            Err(e) => {
                println!("tail err {:?}", e);
                return Poll::Ready(None);
            }
        }
        return Poll::Pending;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tail() {}
}
