Rust library to process payments

#Basics

This cli application builds and is properly formatted with snake_case 
```
cargo run -- transactions.csv > accounts.csv
```

#Completeness

All 5 kinds of transactions Withdraw, Deposit, Dispute, Resolution and Chargebacks are
handled by creating a transaction mod which consists of enum of 5 different transaction type.

#Correctness

Application system is strongly typed to ensure correctness
Inside the client mod everything kind of transaction is first processed before adding it to the application state.
During proccessing all types of transactions are kept into consideration.
Unit tests are written for the mod to check for logical errors.

#Safety and Robustness

Application error handling is done using quick_error crate
Where error types are separated by enums as io,csv and program error

#Efficiency

tinyset::Set is used to store small sets with just the size of one pointer, with no heap storage.
hashbrown::HashMap is because it uses lower memory and more fast

#Maintainability

All logic is separated out into different modules to enable better maintainability and abstraction





