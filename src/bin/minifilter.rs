use std::sync::mpsc::channel;
use unix_fsfilter_rs::shared_def::IOMessage;

fn main() {
    let (tx_iomsgs, rx_iomsgs) = channel::<IOMessage>();
}
