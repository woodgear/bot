#[macro_export]
macro_rules! ret_eq {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    println!(r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val);
                    return Ok(false);
                }

            }
        }
    });
    ($left:expr, $right:expr,) => ({
        $crate::assert_eq!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    println!(r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           std::format_args!($($arg)+));
                    return Ok(false);
                }
            }
        }
    });
}

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
