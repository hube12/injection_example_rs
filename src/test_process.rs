/// Process to be launched in the background to try inject into

fn main() {
    for _ in 0..120 {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
