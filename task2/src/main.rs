use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::sync::RwLock;

pub struct Client {
    id: u64,
    balance: RwLock<usize>,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            balance: RwLock::new(self.balance.read().unwrap().clone()),
        }
    }
}

pub struct Bank {
    id: u64,
    balance: RwLock<usize>,
    clients: Vec<Client>,
    tx: Sender<Transaction>,
    rx: Receiver<Transaction>
}

impl Clone for Bank {
    fn clone(&self) -> Self {
        let (tx, rx) = mpsc::channel::<Transaction>();
        Self {
            id: self.id,
            balance: RwLock::new(self.balance.read().unwrap().clone()),
            clients: self.clients
                .iter()
                .map(|client| Client {
                    id: client.id,
                    balance: RwLock::new(*client.balance.read().unwrap()),
                })
                .collect(),
            tx: self.tx.clone(),
            rx: rx
        }
    }
}

pub struct Transaction {
    from_bank_id: u64,
    to_bank_id: u64,
    from_client_id: u64,
    to_client_id: u64,
    amount: usize
}

impl Clone for Transaction {
    fn clone(&self) -> Self {
        let (tx, rx) = mpsc::channel::<Transaction>();
        Self {
            from_bank_id: self.from_bank_id,
            to_bank_id: self.to_bank_id,
            from_client_id: self.from_client_id,
            to_client_id: self.to_client_id,
            amount: self.amount
        }
    }
}

impl Bank {  
    fn start(&self, tx: Sender<Bank>) {
        while let Ok(transaction) = self.rx.recv() {
            println!("Сумма: {}, от банка {} к банку {}.",
                transaction.amount, transaction.from_bank_id, transaction.to_bank_id);
        
            if transaction.from_bank_id == self.id {
                let from_client = self.clients.iter().find(|&client| client.id == transaction.from_client_id).unwrap();
                println!("Счёт отправителя до транзанкции: {}", from_client.balance.read().unwrap());
                {
                    let mut from = from_client.balance.write().unwrap();
                    *from -= transaction.amount;
                }
                println!("Счёт отправителя после транзанкции: {}", from_client.balance.read().unwrap());
            };

            if transaction.to_bank_id == self.id {
                let to_client = self.clients.iter().find(|&client| client.id == transaction.to_client_id).unwrap();
                println!("Счёт получателя до транзанкции: {}", to_client.balance.read().unwrap());
                {
                    let mut to = to_client.balance.write().unwrap();
                    *to += transaction.amount;
                }
                println!("Счёт получателя после транзанкции: {}", to_client.balance.read().unwrap());
            };

            tx.send(self.clone()).unwrap();
        }
    }
}

fn send_transaction(bank: Bank, tx_bank: Sender<Bank>, tx: Sender<Transaction>, transaction: Transaction) {
    let from_client = bank.clients.iter().find(|&client| client.id == transaction.from_client_id).unwrap();
    let from = from_client.balance.read().unwrap();
    let self_tx = bank.tx.clone();
    let transaction1 = transaction.clone();
    if *from >= transaction.amount {
        if transaction.from_bank_id == transaction.to_bank_id {
            println!("Внутрибанковая транзанкция");
        } else {
            println!("Межбанковская транзанкция");
            thread::spawn(move || {
                self_tx.send(transaction).unwrap();
            });
        }
        thread::spawn(move || {
            tx.send(transaction1).unwrap();
        });
    } else {
        println!("Не хватает средств на счете отправителя");
        tx_bank.send(bank.clone()).unwrap();
    } 
}

fn main() {
    let (tx_bank1, rx_bank1) = mpsc::channel::<Transaction>();
    let (tx_bank2, rx_bank2) = mpsc::channel::<Transaction>();
    
    let client1 = Client {
        id: 1,
        balance: RwLock::new(100),
    };
    let client2 = Client {
        id: 2,
        balance: RwLock::new(200),
    };
    let client3 = Client {
        id: 3,
        balance: RwLock::new(300),
    };

    let bank1 = Bank {
        id: 1,
        balance: RwLock::new(0),
        clients: vec![client1, client2],
        tx: tx_bank1,
        rx: rx_bank1
    }; 
    let bank2 = Bank {
        id: 2,
        balance: RwLock::new(0),
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
    let transaction4 = Transaction {
        from_bank_id: 2,
        to_bank_id: 1,
        from_client_id: 3,
        to_client_id: 1,
        amount: 40,
    };
    let transaction5 = Transaction {
        from_bank_id: 1,
        to_bank_id: 1,
        from_client_id: 1,
        to_client_id: 2,
        amount: 10,
    };
    let transaction6 = Transaction {
        from_bank_id: 1,
        to_bank_id: 1,
        from_client_id: 1,
        to_client_id: 2,
        amount: 60,
    };

    let tx_bank1_copy = bank1.tx.clone();
    let tx_bank2_copy = bank2.tx.clone();

    let (tx1, rx1) = mpsc::channel::<Bank>();
    let (tx2, rx2) = mpsc::channel::<Bank>();

    let tx1_clone = tx1.clone();
    let tx2_clone = tx2.clone();

    tx1.send(bank1.clone()).unwrap();
    tx2.send(bank2.clone()).unwrap();

    thread::spawn(move || {
        bank1.start(tx1_clone);
    });
    thread::spawn(move || {
        bank2.start(tx2_clone);
    });

    let mut bank = rx1.recv().unwrap();
    let mut bank2 = rx2.recv().unwrap();
    send_transaction(bank.clone(), tx1.clone(), tx_bank1_copy.clone(), transaction1);
    bank = rx1.recv().unwrap();
    send_transaction(bank.clone(), tx1.clone(), tx_bank1_copy.clone(), transaction2);
    bank = rx1.recv().unwrap();
    send_transaction(bank.clone(), tx1.clone(), tx_bank2_copy.clone(), transaction3);
    bank2 = rx2.recv().unwrap();
    bank = rx1.recv().unwrap();
    send_transaction(bank2.clone(), tx2.clone(), tx_bank1_copy.clone(), transaction4);
    bank = rx1.recv().unwrap();
    send_transaction(bank.clone(), tx1.clone(), tx_bank1_copy.clone(), transaction5);
    bank = rx1.recv().unwrap();
    send_transaction(bank.clone(), tx1.clone(), tx_bank1_copy.clone(), transaction6);
    println!("{}", bank.clients[0].balance.read().unwrap());
    println!("{}", bank.clients[1].balance.read().unwrap());
    bank2 = rx2.recv().unwrap();
    println!("{}", bank2.clients[0].balance.read().unwrap());
}
