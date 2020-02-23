
pub fn md5_file<P:AsRef<std::path::Path>>(p:P) ->Result<String,failure::Error> {
    let buff  = std::fs::read(p)?;
    return Ok(md5(&buff));
}

pub fn md5(buff:&[u8]) ->String{
    let digest = md5::compute(buff);    
    return format!("{:?}",digest);
}