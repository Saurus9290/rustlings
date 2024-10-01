use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    // Thread to send the first half of the queue
    thread::spawn(move || {
        for val in q.first_half {
            println!("Sending {val:?}");
            tx1.lock().unwrap().send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    // Thread to send the second half of the queue
    thread::spawn(move || {
        for val in q.second_half {
            println!("Sending {val:?}");
            tx2.lock().unwrap().send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can experiment with the code here if desired.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();
        let queue_length = queue.length;

        let tx = Arc::new(Mutex::new(tx));
        send_tx(queue, Arc::clone(&tx));

        let mut total_received: u32 = 0;
        for received in rx {
            println!("Got: {received}");
            total_received += 1;
        }

        println!("Number of received values: {total_received}");
        assert_eq!(total_received, queue_length);
    }
}
