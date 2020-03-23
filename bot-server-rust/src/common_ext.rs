pub use failure::ResultExt;
type BoxStdError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait CustomFailureExt<T> {
    fn to_box_std_err(self) -> Result<T, BoxStdError>;
    fn to_ws_err(self) -> Result<T, ws::Error>;
}

impl<T> CustomFailureExt<T> for Result<T, failure::Error> {
    fn to_box_std_err(self) -> Result<T, BoxStdError> {
        self.compat().map_err(|e| {
            let std_err: BoxStdError = Box::new(e);
            std_err
        })
    }
    fn to_ws_err(self) -> Result<T, ws::Error> {
        self.to_box_std_err()
            .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(e), ""))
    }
}
