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
use async_std::stream::{self, Timeout};
use std::time::{Duration, Instant};
type TimeEvent = Result<String, TimeoutError>;
pub struct StreamAssert {
    pub should_see: Vec<String>,
    pub should_not_see: Vec<String>,
    pub util_see: String,
    pub timeout: Duration,
    pub stream: Pin<Box<Stream<Item = TimeEvent>>>,
    pub elapsed: Duration, //timeout的实现逻辑应当在外部
    pub start: Instant,
    util_see_flag: bool,
    should_see_map: std::collections::HashMap<usize, bool>,
}

impl StreamAssert {
    pub fn new<S: 'static + Stream<Item = String>>(
        should_see: Vec<&str>,
        should_not_see: Vec<&str>,
        util_see: &str,
        stream: S,
        timeout: u64,
    ) -> Self {
        Self {
            should_see: should_see.iter().map(|s| s.to_string()).collect(),
            should_not_see: should_not_see.iter().map(|s| s.to_string()).collect(),
            timeout: Duration::from_secs(timeout),
            util_see: util_see.to_string(),
            stream: Box::pin(stream.timeout(Duration::from_secs(3))),
            elapsed: Duration::from_secs(0),
            start: Instant::now(),
            should_see_map: std::collections::HashMap::new(),
            util_see_flag: false,
        }
    }
    fn am_i_seed_all_i_need_to_sedd(&self) -> bool {
        let should_see_count = self.should_see_map.values().fold(0, |pre, e| {
            if *e {
                return pre + 1;
            }
            return pre;
        });
        println!("{} {}",should_see_count,self.should_see.len());
        if should_see_count == self.should_see.len() {
            return true;
        }
        return false;
    }
    fn step(&mut self, event: Option<String>) -> Option<bool> {
        self.elapsed = self.start.elapsed();
        if let Some(line) = event {
            println!("line {}", line);
            if self
                .should_not_see
                .iter()
                .any(|keyword| line.contains(keyword))
            {
                return Some(false);
            }
            for (index, keyword) in self.should_see.iter().enumerate() {
                println!("xxx {} | {}",line,keyword);
                if line.contains(keyword) {
                    println!("i see {:?}",keyword);
                    self.should_see_map.insert(index, true);
                }
            }
            if line.contains(&self.util_see) {
                self.util_see_flag = true;
                return Some(self.am_i_seed_all_i_need_to_sedd());
            }
        }
        if self.elapsed > self.timeout {
            return Some(self.am_i_seed_all_i_need_to_sedd() && self.util_see_flag);
        }
        return None;
    }
}
impl Future for StreamAssert {
    type Output = Result<bool, failure::Error>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this: &mut Self = &mut *self;
        loop {
            let ret = match futures::ready!(this.stream.as_mut().poll_next(cx)) {
                //tail event
                Some(Ok(line)) => Some(line),
                //timeout
                Some(Err(_)) => None,
                //stream end
                None => {
                    println!("{:?}", "stream over??");
                    None
                }
            };
            if let Some(flag) = this.step(ret) {
                return Poll::Ready(Ok(flag));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tail() {}
}
