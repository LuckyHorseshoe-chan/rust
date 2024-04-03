use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::sync::{Arc, RwLock};

pub struct Client {
    id: u64,
    balance: Arc<RwLock<usize>>,
}

pub struct Bank {
    id: u64,
    balance: Arc<RwLock<usize>>,
    clients: Vec<Client>,
    tx: Sender<Transaction>,
    rx: Receiver<Transaction>
}

pub struct Transaction {
    from_bank_id: u32,
    to_bank_id: u32,
    from_client_id: u32,
    to_client_id: u32,
    amount: usize
}