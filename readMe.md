Rust library to process payments

## Basics and Usage

This cli application builds and is properly formatted with snake_case 
```
cargo run -- transactions.csv > accounts.csv
```

## Completeness

All 5 types of transactions Withdraw, Deposit, Dispute, Resolution and Chargebacks are <br>
handled by creating a transaction mod which consists of enum of 5 different transaction types.

## Correctness

Application system is strongly typed to ensure correctness  <br>
Inside the client mod every type of transaction is first processed before adding it to the application state.<br>
During proccessing all types of transactions are kept into consideration.<br>
Unit tests are written for the mod to check for logical errors.<br>

## Safety and Robustness

Application error handling is done using quick_error crate<br>
Where error types are separated by enums as io,csv and program error<br>

## Efficiency

tinyset::Set is used to store small sets with just the size of one pointer, with no heap storage.<br>
hashbrown::HashMap is because it uses lower memory and more fast<br>

## Maintainability

All logic is separated out into different modules to enable better maintainability and abstraction





