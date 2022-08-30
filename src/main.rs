use payment_processing_engine::{get_args, process_transactions,  print_client_account_balances};

fn main() {
    
    let args= match get_args(){
        Ok(state) => state,
        Err(e) => panic!("Error while parsing args{}", e)
    };

    // println!("File name:{} {:?}", args.filename(), args.clients);


    

    let process = match process_transactions(args){
        Ok(process) => process,
        Err(e) => panic!("Error processing transactions: {}", e)
    };
    // println!("After processing {:?}", process);

    let _output = match print_client_account_balances(process){
        Ok(_) => "Successfully created the file",
        Err(_) => "Error creating the file"
    };

   
 
}
