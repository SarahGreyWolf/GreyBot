use greybot::thread_pool::ThreadPool;
use greybot::client::Client;

fn main() {
    let pool = ThreadPool::new(5);

    for _ in 0..10 {
        pool.execute(|| {
            println!("{}", 10+6*4);
        });
    }

    let client = Client::new();
}
