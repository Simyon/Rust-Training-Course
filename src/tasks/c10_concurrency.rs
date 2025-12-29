// This chapter is dedicated to the concurrency.

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// THREADS & JOIN
// ================================================================================================

// ----- 1 --------------------------------------
// Spawn multiple threads to calculate squares of the provided numbers and collect the results.

pub fn calculate_squares(input_numbers: Vec<i32>) -> Vec<i32> {
    let (tx, rx): (Sender<(usize, i32)>, Receiver<(usize, i32)>) = mpsc::channel();

    let mut handles = Vec::with_capacity(input_numbers.len());

    for (idx, n) in input_numbers.into_iter().enumerate() {
        let tx_clone = tx.clone();
        handles.push(thread::spawn(move || {
            let sq = n * n;
            let _ = tx_clone.send((idx, sq));
        }));
    }

    drop(tx);

    let mut results = vec![0; handles.len()];
    for (idx, sq) in rx.iter() {
        results[idx] = sq;
    }

    for h in handles {
        let _ = h.join();
    }

    results
}

// ----- 2 --------------------------------------
// Implement a `parallel_prime_check` function that splits work across multiple threads.

fn is_prime(number: u64) -> bool {
    if number <= 1 {
        return false;
    }
    for divisor in 2..=((number as f64).sqrt() as u64) {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

/// Inputs:
/// - `numbers` - a `u64` vector of values which should be checked.
/// - `number_of_threads` - a number of threads you must use to *efficiently* distribute the values
///   from the numbers vector.
///
/// Outputs:
/// - `Vec<(u64, bool)>` is a vector of the provided values along with the boolean flag whether this
///   value is prime.
pub fn parallel_prime_check(numbers: Vec<u64>, number_of_threads: usize) -> Vec<(u64, bool)> {
    use std::cmp::min;
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::thread;

    let len = numbers.len();
    if len == 0 {
        return Vec::new();
    }

    let threads = min(len, number_of_threads.max(1));
    let shared = Arc::new(numbers);

    let (tx, rx) = mpsc::channel::<(usize, (u64, bool))>();
    let mut handles = Vec::with_capacity(threads);

    let chunk = (len + threads - 1) / threads;

    for t in 0..threads {
        let tx = tx.clone();
        let shared = Arc::clone(&shared);

        let start = t * chunk;
        let end = min(start + chunk, len);

        if start >= end {
            continue;
        }

        handles.push(thread::spawn(move || {
            for i in start..end {
                let n = shared[i];
                let p = is_prime(n);
                let _ = tx.send((i, (n, p)));
            }
        }));
    }

    drop(tx);

    let mut out = vec![(0u64, false); len];
    for (i, pair) in rx.iter() {
        out[i] = pair;
    }

    for h in handles {
        let _ = h.join();
    }

    out
}

// MPSC CHANNELS
// ================================================================================================

// ----- 3 --------------------------------------
// Compute the factorial for each value in the provided vector.
// Use a separate thread for each computation.
// Send the factorial results to the main thread using a channel transmitter.
// Using a channel receiver, collect the resulting factorial values into a vector and return it from
// the function.

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

pub fn parallel_factorials(numbers: Vec<u32>) -> Vec<u32> {
    let (tx, rx): (Sender<(usize, u32)>, Receiver<(usize, u32)>) = mpsc::channel();

    for (index, &n) in numbers.iter().enumerate() {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let result = factorial(n);
            let _ = tx_clone.send((index, result));
        });
    }

    drop(tx);

    let mut results_with_index: Vec<(usize, u32)> = rx.iter().collect();

    results_with_index.sort_by_key(|&(i, _)| i);
    results_with_index.into_iter().map(|(_, val)| val).collect()
}

// MUTEX + ARC
// ================================================================================================

// ----- 4 --------------------------------------
// Implement a `SharedCounter` struct with one `value: ?<i32>` field and methods:
// - `pub fn new(initial_value: i32) -> Self`, which creates a new instance of the `SharedCounter`.
// - `pub fn increment(&self)` which will increment the internal value.
// - `pub fn get_value(&self) -> i32` which will return the internal value.
//
// Notice that these methods could be called from the several threads at the same time. Use `Arc`
// and `Mutex` where needed.

#[derive(Clone)]
pub struct SharedCounter {
    value: Arc<Mutex<i32>>,
}

impl SharedCounter {
    pub fn new(initial_value: i32) -> Self {
        SharedCounter {
            value: Arc::new(Mutex::new(initial_value)),
        }
    }

    pub fn increment(&self) {
        let mut guard = self.value.lock().unwrap();
        *guard += 1;
    }

    pub fn get_value(&self) -> i32 {
        let guard = self.value.lock().unwrap();
        *guard
    }
}

// ----- 5 --------------------------------------
// Simulate a bank account system with concurrent deposits and withdrawals.
//
// Implement a `BankAccount` struct with one `balance: ?<i32>` field and methods:
// - `pub fn new(initial_balance: i32) -> Self`, which creates a new instance of the `BankAccount`.
// - `pub fn deposit(&self, amount: i32)` which adds the provided amount to the balance.
// - `pub fn withdraw(&self, amount: i32) -> bool` which attempts to remove the provided amount from
//   the balance. If the balance have sufficient funds, it removes the provided amount and returns
//   `true`, otherwise returns `false`.
// - `pub fn get_balance(&self)` which returns the current balance.
//
// Notice that these methods could be called from the several threads at the same time. Use `Arc`
// and `Mutex` where needed.

#[derive(Clone)]
pub struct BankAccount {
    balance: Arc<Mutex<i32>>,
}

impl BankAccount {
    pub fn new(initial_balance: i32) -> Self {
        Self {
            balance: Arc::new(Mutex::new(initial_balance)),
        }
    }

    pub fn deposit(&self, amount: i32) {
        let mut guard = self.balance.lock().unwrap();
        *guard += amount;
    }

    pub fn withdraw(&self, amount: i32) -> bool {
        let mut guard = self.balance.lock().unwrap();
        if *guard >= amount {
            *guard -= amount;
            true
        } else {
            false
        }
    }

    pub fn get_balance(&self) -> i32 {
        let guard = self.balance.lock().unwrap();
        *guard
    }
}

// FINAL BOSS: CHANNELS + MUTEX + ARC
// ================================================================================================

// ----- 6 --------------------------------------
// Implement a work queue where multiple workers consume tasks and send results back (a simple task
// distribution system).
//
// You will need to implement two procedures:
// - `worker(id: usize, task_receiver: ?<Receiver<i32>>, result_sender: ?<Sender<(usize, i32)>>)`,
//   which has the ID of the worker, the task receiver, which waits for the task to be provided to
//   this worker, and the result sender, which sends the computed result back to the main thread.
//   This procedure should:
//   - Loop over all incoming tasks from `task_receiver`.
//   - For each task, compute the square of the value this task provided.
//   - Send the result back via `result_sender` along with the worker’s ID:
//    `(worker_id, result_value)`.
//   - Decide by your own whether this procedure should return something or not, and if should --
//    what exactly.
//   - Use `Arc` or `Mutex` if needed.
// - `run_work_queue(tasks: Vec<i32>, number_of_workers: usize) -> Vec<(usize, i32)>` which has the
//   vector of tasks (just values, which square we should compute) and the total number of workers
//   which should be spawned. It returns the vector of worker IDs (`usize`) and the resulting value
//   computed by this worker (`i32`). This procedure should:
//   - Create two channels: for sending tasks to workers and for collecting results from workers.
//   - For each worker spawn a thread which runs the worker function, consuming tasks and sending
//     results.
//   - Send each task from the input list into the task_sender.
//   - Collect all results from the result_receiver into a vector and return it.

fn worker(
    worker_id: usize,
    task_receiver: Arc<Mutex<Receiver<i32>>>,
    result_sender: Sender<(usize, i32)>,
) {
    loop {
        let task = {
            let rx = task_receiver.lock().unwrap();
            rx.recv()
        };

        match task {
            Ok(v) => {
                let result = v * v;
                let _ = result_sender.send((worker_id, result));
            },
            Err(_) => break,
        }
    }
}

pub fn run_work_queue(tasks: Vec<i32>, number_of_workers: usize) -> Vec<(usize, i32)> {
    let task_count = tasks.len();

    let (task_sender, task_receiver) = channel::<i32>();
    let (result_sender, result_receiver) = channel::<(usize, i32)>();

    let task_receiver = Arc::new(Mutex::new(task_receiver));

    let mut handles = Vec::with_capacity(number_of_workers);
    for id in 0..number_of_workers {
        let rx = Arc::clone(&task_receiver);
        let tx = result_sender.clone();
        handles.push(thread::spawn(move || worker(id, rx, tx)));
    }

    drop(result_sender);

    for task in tasks {
        task_sender.send(task).unwrap();
    }
    drop(task_sender);

    let mut results = Vec::with_capacity(task_count);
    for _ in 0..task_count {
        results.push(result_receiver.recv().unwrap());
    }

    for h in handles {
        h.join().unwrap();
    }

    results
}
