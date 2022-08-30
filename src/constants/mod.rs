lazy_static! {
    
    pub static ref STANDARD_INPUT_HEADER: csv::ByteRecord = csv::ByteRecord::from(
        vec!["type", "client", "tx", "amount"]
    );

    pub static ref MINIMAL_INPUT_HEADER: csv::ByteRecord = csv::ByteRecord::from(
        vec!["type", "client", "tx"]
    );
}

pub const OUTPUT_HEADER: &[&str; 5] = &["client", "available", "held", "total", "locked"];
