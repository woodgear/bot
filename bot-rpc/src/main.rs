pub struct ExecRreq {

}
pub struct ExecRes {

}

pub struct TailReq {

}
pub enum TailRes {
    Continue(String),
    End,
}

pub struct Bot {

}

pub struct RpcStream<T> {
    t:T,
}

impl Bot {
   async fn exec(req:ExecRreq) -> ExecRes {
       unimplemented!();
   }

   async fn tail(req:TailReq) -> RpcStream<TailRes> {
        unimplemented!();
   }
}
pub fn main() {
    println!("{:?}","ok");
}