use std::sync::mpsc;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::sync::RwLock;
// use task2::Client;
// use task2::Bank;
// use task2::Transaction;

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

impl Bank {  
    fn start(&self) {
        while let Ok(transaction) = self.rx.recv() {
            // Получаем доступ к балансу клиента отправителя
            println!("{}", transaction.from_client_id);
            // let from_client = clients.iter().find(|&client| client.id == transaction.from_client_id).unwrap();
            // let from_balance = from_client.balance.clone();
            
            // Получаем доступ к балансу клиента получателя
            let to_client = self.clients.iter().find(|&client| client.id == transaction.to_client_id).unwrap();
            // let to_balance = dbg!(to_client.balance.read().unwrap().clone());
            
            // Блокируем оба баланса для изменений
            // let mut from = from_balance.write().unwrap();
            let mut to = to_client.balance.write().unwrap();

            *to += transaction.amount;
            println!("Сумма: {}, от банка {} к банку {}.",
                transaction.amount, transaction.from_bank_id, transaction.to_bank_id);
            
            // Проверяем достаточность средств на счете отправителя
            // if *from >= transaction.amount {
            //     // Выполняем перевод между счетами
            //     *from -= transaction.amount;
            //     *to += transaction.amount;
                
            //     dbg!("Сумма: {}, от банка {} к банку {}.",
            //                 transaction.amount, transaction.from_bank_id, transaction.to_bank_id);
            // } else {
            //     println!("Не хватает средств на счете отправителя");
            // } 
        
        }
    }
    
    fn send_transaction(&self, tx: Sender<Transaction>, transaction: Transaction) {
        let from_client = self.clients.iter().find(|&client| client.id == transaction.from_client_id).unwrap();
        let from = from_client.balance.read().unwrap();
        println!("отправитель {}", transaction.from_client_id);
        if *from >= transaction.amount {
            thread::spawn(move || {
                println!("шлём {} рублей", transaction.amount);
                tx.send(transaction).unwrap();
            });
        } else {
            println!("Не хватает средств на счете отправителя");
        } 
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
        from_bank_id: 1,
        to_bank_id: 1,
        from_client_id: 1,
        to_client_id: 2,
        amount: 50,
    };
    let transaction5 = Transaction {
        from_bank_id: 1,
        to_bank_id: 1,
        from_client_id: 1,
        to_client_id: 2,
        amount: 50,
    };
    let transaction6 = Transaction {
        from_bank_id: 1,
        to_bank_id: 1,
        from_client_id: 1,
        to_client_id: 2,
        amount: 50,
    };

    let b1 = bank1.clone();
    let b2 = bank2.clone();

    b1.send_transaction(b1.tx.clone(), transaction1);
    b1.send_transaction(b1.tx.clone(), transaction2);
    b1.send_transaction(b2.tx.clone(), transaction3);
    // b1.send_transaction(b1.tx.clone(), transaction4);
    // b1.send_transaction(b1.tx.clone(), transaction5);
    // b1.send_transaction(b1.tx.clone(), transaction6);

    thread::spawn(move || {
        bank1.start();
    });
    thread::spawn(move || {
        bank2.start();
    });

    // let (tx1, rx1) = mpsc::channel::<Transaction>();
    // let (tx2, rx2) = mpsc::channel::<Transaction>();

    // thread::spawn(move || {
    //     while let Ok(transaction) = rx1.recv() {
    //         // Получаем доступ к балансу клиента отправителя
    //         println!("{}", transaction.amount);
    //     }
    // });

    // thread::spawn(move || {
    //     while let Ok(transaction) = rx2.recv() {
    //         // Получаем доступ к балансу клиента отправителя
    //         println!("{}", transaction.amount);
    //     }
    // });

    // tx1.send(transaction1).unwrap();
    // tx1.send(transaction2).unwrap();
    // tx2.send(transaction3).unwrap();
}
