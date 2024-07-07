// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{sync::Arc, sync::Mutex, thread, time::Duration};

struct JobStatus {
    jobs_done: Mutex<u32>,
}

fn main() {
    let status: Arc<JobStatus> = Arc::new(JobStatus {
        jobs_done: Mutex::new(0),
    });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            let mut jobs_done: std::sync::MutexGuard<u32> = status_shared.jobs_done.lock().unwrap();
            *jobs_done += 1;
            
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Jobs done: {}", status.jobs_done.lock().unwrap());
}
