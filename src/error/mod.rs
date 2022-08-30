use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Custom(err: String) {
            from()
            from(err: &str) -> (err.into())
            display("✘ Program Error!\n✘ {err}")
        }
        Io(err: std::io::Error) {
            from()
            display("✘ I/O Error!\n✘ {err}")
        }
        Csv(err: csv::Error) {
            from()
            display("✘ CSV Error!\n✘ {err}")
        }
    }
}
