use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread;
use std::sync::{Arc, RwLock};
// use task2::Client;
// use task2::Bank;
// use task2::Transaction;

#[derive(Clone)]
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
    from_bank_id: u64,
    to_bank_id: u64,
    from_client_id: u64,
    to_client_id: u64,
    amount: usize
}

impl Bank {  
    fn start(&self) {
        let balance = self.balance.clone();
        let rx = self.rx.clone();
        let clients = self.clients.clone();
        
        thread::spawn(move || {
            while let Ok(transaction) = rx.recv() {
                // Получаем доступ к балансу клиента отправителя
                let from_client = clients.iter().find(|&client| client.id == transaction.from_client_id).unwrap();
                let from_balance = from_client.balance.clone();
                
                // Получаем доступ к балансу клиента получателя
                let to_client = clients.iter().find(|&client| client.id == transaction.to_client_id).unwrap();
                let to_balance = to_client.balance.clone();
                
                // Блокируем оба баланса для изменений
                let mut from = from_balance.write().unwrap();
                let mut to = to_balance.write().unwrap();
                
                // Проверяем достаточность средств на счете отправителя
                if *from >= transaction.amount {
                    // Выполняем перевод между счетами
                    *from -= transaction.amount;
                    *to += transaction.amount;
                    
                    println!("Сумма: {}, от банка {} к банку {}.",
                                transaction.amount, transaction.from_bank_id, transaction.to_bank_id);
                } else {
                    println!("Не хватает средств на счете отправителя");
                } 
            }
        });
    }
    
    fn send_transaction(&self, tx: Sender<Transaction>, transaction: Transaction) {
        tx.send(transaction).unwrap();
    }
}

fn main() {
    let (tx_bank1, rx_bank1) = unbounded();
    let (tx_bank2, rx_bank2) = unbounded();
    
    let client1 = Client {
        id: 1,
        balance: Arc::new(RwLock::new(100)),
    };
    let client2 = Client {
        id: 2,
        balance: Arc::new(RwLock::new(200)),
    };
    let client3 = Client {
        id: 3,
        balance: Arc::new(RwLock::new(300)),
    };

    let bank1 = Bank {
        id: 1,
        balance: Arc::new(RwLock::new(0)),
        clients: vec![client1, client2],
        tx: tx_bank1,
        rx: rx_bank1
    }; 
    let bank2 = Bank {
        id: 2,
        balance: Arc::new(RwLock::new(0)),
        clients: vec![client3],
        tx: tx_bank2,
        rx: rx_bank2
    }; 

    let transaction1 = Transaction {
        from_bank_id: 1,
        to_bank_id: 1,
        from_client_id: 1,
        to_client_id: 2,
        amount: 50,
    };
    let transaction2 = Transaction {
        from_bank_id: 1,
        to_bank_id: 1,
        from_client_id: 2,
        to_client_id: 1,
        amount: 100,
    };
    let transaction3 = Transaction {
        from_bank_id: 1,
        to_bank_id: 2,
        from_client_id: 1,
        to_client_id: 3,
        amount: 150,
    };

    bank1.start();
    bank2.start();

    bank1.send_transaction(bank1.tx.clone(), transaction1);
    bank1.send_transaction(bank1.tx.clone(), transaction2);
    bank1.send_transaction(bank2.tx.clone(), transaction3);
}
