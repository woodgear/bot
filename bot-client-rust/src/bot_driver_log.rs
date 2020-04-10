
pub trait DriverLog {
    
}

pub async fn init(url:String) -> Result<(),failure::Error> {
    println!("{:?}",url);
}
