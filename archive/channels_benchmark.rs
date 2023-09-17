extern crate termion;

use flume;

fn thread1(send: flume::Sender<(Instant, UniversalGamepad)>) -> ! {
    loop {
        let now = Instant::now();
        send.send((now, UniversalGamepad::nothing_pressed()));

        thread::sleep(Duration::from_millis(2));
    }
}

fn thread2(recv: flume::Receiver<(Instant, UniversalGamepad)>) -> ! {
    let mut all: Vec<Duration> = Vec::new();

    loop {
        let (t1, gamepad) = recv.recv().unwrap();
        let now = Instant::now();

        all.push(now - t1);

        let mut avg: Duration = Duration::new(0, 0);
        let len = all.len() as u32;

        for x in &all {
            avg += *x;
        }
        avg = avg / len;

        println!("{}", termion::clear::All);
        println!("avg diff: {:3.3?}  count {}", avg, len);
    }
}

fn main() {
    let (s, r): (flume::Sender<(Instant, UniversalGamepad)>, flume::Receiver<(Instant, UniversalGamepad)>) = flume::unbounded();

    let t1 = thread::Builder::new()
        .name("input".to_string())
        .spawn(move || thread1(s))
        .expect("creating input thread failed");
    let t2 = thread::Builder::new()
        .name("input".to_string())
        .spawn(move || thread2(r))
        .expect("creating input thread failed");

    t1.join().unwrap();
    t2.join().unwrap();
    exit(0);
}
