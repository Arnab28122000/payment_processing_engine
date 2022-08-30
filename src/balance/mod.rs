/// Balance of the client
#[derive(Debug, Default, PartialEq)]
pub struct Balance {
    
    pub available: f32,

    
    pub held: f32,
}

impl Balance {
    
    pub fn new(available: f32) -> Self {
        Balance { available, held: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    

    #[test]
    fn should_create_new_balance() {
        let balance = Balance::new(7.908);
        assert_eq!(balance, Balance {
            available: 7.908,
            held: 0.0
        });
    }
}
