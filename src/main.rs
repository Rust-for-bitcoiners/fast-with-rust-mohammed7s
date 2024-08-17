use std::env;
use bitcoincore_rpc::{Auth, Client, RpcApi};
//use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread; 
//use rayon::prelude::*; // Already present, but make sure it's correctly in your imports



lazy_static::lazy_static! {
    static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password: String =
            env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap()
    };
}

// fn main() {
//     // Get the current block count
//     let block_count = RPC_CLIENT.get_block_count().unwrap();

//     // Create a shared counter to accumulate the transaction counts
//     let tx_counter = Arc::new(AtomicU64::new(0));

//     // Number of threads to use (based on your CPU)
//     let num_threads = 4;

//     // Parallel iteration over block heights
//     (0..block_count)
//         .into_par_iter()
//         .for_each(|block_height| {
//             if block_height % num_threads == 0 {
//                 let block_hash = RPC_CLIENT.get_block_hash(block_height).unwrap();
//                 let block = RPC_CLIENT.get_block(&block_hash).unwrap();
//                 let tx_count = block.txdata.len() as u64;
//                 tx_counter.fetch_add(tx_count, Ordering::SeqCst);
//             }
//         });

//     println!("Total number of transactions: {}", tx_counter.load(Ordering::SeqCst));
// }


fn main() {
    let block_count = RPC_CLIENT.get_block_count().unwrap(); 
    let num_threads = 4; 
    let tx_counter = Arc::new(Mutex::new(0u64)); 

    let mut handles = vec![]; 
    for i in 0..num_threads {
        let tx_counter = Arc::clone(&tx_counter);
        let handle = thread::spawn(move || {
            for block_height in (i..block_count).step_by(num_threads as usize){
                let block_hash = RPC_CLIENT.get_block_hash(block_height).unwrap(); 
                let block = RPC_CLIENT.get_block(&block_hash).unwrap(); 
                let tx_count = block.txdata.len() as u64; 

                let mut counter = tx_counter.lock().unwrap(); 
                *counter += tx_count; 
            }
        }); 
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); 
    }
    println!("Total number of transactions: {}", *tx_counter.lock().unwrap()); 
}