struct MasterCard {
    number: u8,
    verification: u8,
}
struct Visa {
    number: u32
}
struct WesternUnion {
    name: String,
    verification: u8,
}
struct BitCredit{
    btcnumber: u32
}

trait CreditCharge {
    fn charge_with_id(&self, id: u32) -> bool;
}

impl CreditCharge for WesternUnion {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 3 == (self.verification as u32) % 3
    }
}

impl CreditCharge for BitCredit {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 2 == self.btcnumber % 2
    }
}

pub fn test_traits() {
    let card = BitCredit { btcnumber: 1024 };
    let code = 4096;
    if card.charge_with_id(code) {
        println!("Success!");
    } else {
        println!("Failure!");
    }
}
