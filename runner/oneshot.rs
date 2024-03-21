/*
* From -> https://docs.rs/futures/latest/futures/channel/oneshot/fn.channel.html (slightly edited)
*
* Requires:
*   $ runner --add futures
*
* Shows:
*   - Communicating between two threads, using 'futures::channel::oneshot'
*/
use futures::channel::oneshot;

let (sender, receiver) = oneshot::channel::<i32>();

thread::spawn(|| {
    println!("THREAD: sleeping zzz...");
    thread::sleep(Duration::from_millis(1000));
    println!("THREAD: i'm awake! sending.");
    sender.send(3).unwrap();
});

println!("MAIN: doing some useful stuff");

futures::executor::block_on(async {
    println!("MAIN: waiting for msg...");
    println!("MAIN: got: {:?}", receiver.await)
});
