pub async fn one_shot(
    url: &str,
    req: bot::protocol::ServerAst,
) -> Result<bot::protocol::ServerResponse, failure::Error> {
    use async_tungstenite::{async_std::connect_async, tungstenite::Message};
    use bot::protocol::*;
    use futures::prelude::*;

    let (mut ws_stream, _) = connect_async(url).await?;
    let bin_msg = req.into_binary()?;

    ws_stream.send(Message::Binary(bin_msg)).await?;

    let msg = ws_stream
        .next()
        .await
        .ok_or_else(|| failure::err_msg("didn't receive anything"))??;
    let msg_bin = {
        if let Message::Binary(bin) = msg {
            bin
        } else {
            return Err(failure::err_msg("expect a bin msg but find other"));
        }
    };
    let ret = ServerResponse::from_binary(&msg_bin)?;
    Ok(ret)
}
