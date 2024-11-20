use tokio::sync::mpsc;
pub type Tx<T> = mpsc::UnboundedSender<T>;
pub type Rx<T> = mpsc::UnboundedReceiver<T>;
pub type UiTx = Tx<UiMessage>;
pub type UiRx = Rx<UiMessage>;
#[derive(Debug)]
pub enum UiMessage {}
