use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

static mut SENDER: Option<Sender<u32>> = None;
static mut RECEIVER: Option<Arc<Receiver<u32>>> = None;

pub fn create_channel() -> (Sender<u32>, Arc<Receiver<u32>>) {
    let (tx, rx) = channel();
    let arc_rx = Arc::new(rx);
    unsafe {
        SENDER = Some(tx.clone());
        RECEIVER = Some(arc_rx.clone());
    }
    (tx, arc_rx)
}

pub fn get_sender() -> Sender<u32> {
    unsafe { SENDER.as_ref().unwrap().clone() }
}

pub fn get_receiver() -> Arc<Receiver<u32>> {
    unsafe { RECEIVER.as_ref().unwrap().clone() }
}
