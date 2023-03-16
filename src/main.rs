use std::{env, sync::Arc};

use futures::future;
// use futures::future;
use tokio::{sync::broadcast, task::JoinHandle};
#[derive(Clone, Debug)]
pub enum Status {
    Yes,
    No,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub block_hash: Vec<u8>,
    pub block_number: u64,
    pub gas_price: Option<u64>,
    pub parent_block_hash: [u8; 334],
    pub sequencer_address: Option<u64>,
    pub state_root: [u8; 32],
    pub status: Status,
    pub starknet_version: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct SyncEvent {
    pub block: Box<Block>,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut times: usize = 10;

    if args.get(2).is_some() {
        times = args[2].parse::<usize>().unwrap();
    }

    let listeners = if args.get(1).unwrap() == "arc" {
        let res = broadcast_arc(times);
        ("arc", res.0, res.1)
    } else if args.get(1).unwrap() == "num" {
        let res = broadcast_num(times);
        ("num u8", res.0, res.1)
    } else if args.get(1).unwrap() == "dupe" {
        let res = dupe_struct(times);
        ("duplicating", res.0, res.1)
    } else {
        let res = broadcast_struct(times);
        ("struct", res.0, res.1)
    };

    future::join_all(listeners.1).await;

    println!("");
    println!("-----------------------------------------------");
    println!("Broadcasting: {}", listeners.0);
    println!("Listeners: {}", listeners.2);
    println!("-----------------------------------------------");
    println!("");
}

fn dupe_struct(times: usize) -> (Vec<JoinHandle<()>>, usize) {
    let mut a = 0;
    let mut handles = vec![];
    while a < times {
        let event_data = get_event();
        handles.push(tokio::spawn(async move {
            format!("{:#?}", event_data);
        }));
        a += 1;
    }
    (vec![], times)
}

fn broadcast_arc(times: usize) -> (Vec<JoinHandle<()>>, usize) {
    let event_data = get_event();
    broadcast_base(Arc::new(event_data), times)
}

fn broadcast_struct(times: usize) -> (Vec<JoinHandle<()>>, usize) {
    let event_data = get_event();
    broadcast_base(event_data, times)
}

fn broadcast_num(times: usize) -> (Vec<JoinHandle<()>>, usize) {
    let broadcast_data: u8 = 2;
    broadcast_base(broadcast_data, times)
}

fn broadcast_base<Type: 'static + Clone + Send>(
    event_data: Type,
    times: usize,
) -> (Vec<JoinHandle<()>>, usize) {
    let tx: broadcast::Sender<Type> = broadcast::channel(10000).0;
    let mut a = 0;
    let mut handles = vec![];
    while a < times {
        let mut rx = tx.subscribe();
        handles.push(tokio::spawn(async move {
            let _res = rx.recv().await.unwrap();
            // print!("{a}, ");
        }));
        a += 1;
    }

    (handles, tx.send(event_data).ok().unwrap())
}

fn get_event() -> SyncEvent {
    SyncEvent {
        block: Box::new(Block {
            block_hash: vec![
                21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4,
                53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167,
                3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45,
                45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32,
                4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32,
                32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12,
                1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43,
                21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4,
                53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167,
                3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45,
                45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32,
                4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32,
                32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12,
                1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43,
                21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4,
            ],
            block_number: 22345,

            gas_price: None,
            parent_block_hash: [
                1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43,
                21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4,
                53, 5, 45, 43, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12,
                12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45,
                43, 21, 2, 12, 12, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43,
                167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5,
                45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23,
                32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2,
                32, 32, 23, 32, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5,
                45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23,
                32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2,
                32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45, 43, 21, 2, 12,
                12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5, 4, 53, 5, 45,
                43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 43, 167, 3, 54, 5,
                4, 53, 5, 45, 43, 21, 2, 12, 12, 1, 21, 2, 32, 32, 23, 32, 4, 34, 5, 45, 45, 4, 3,
            ],
            sequencer_address: Some(2345),
            state_root: *b"asdlfkjhrieugnvdliufgnlkdajnrlun",
            status: Status::No,
            starknet_version: None,
        }),
    }
}
