#![allow(dead_code)]

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

impl CreditCharge for MasterCard {
    fn charge_with_id(&self, id: u32) -> bool {
        self.number + self.verification % (id % 256) as u8 == 0
    }
}

impl CreditCharge for Visa {
    fn charge_with_id(&self, id: u32) -> bool {
        id != 1026
    }
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

fn transact<Q: CreditCharge>(card: Q) {
    let id = 4096;
    if card.charge_with_id(id) {
        println!("Success in transact!");
    } else {
        panic!("Invalid code in transact!");
    }
}

pub fn test_traits() {
    let card1 = BitCredit { btcnumber: 1024 };
    let code = 4096;
    if card1.charge_with_id(code) {
        println!("Success!");
    } else {
        println!("Failure!");
    }
    // Generic function
    transact(card1);

    let card2 = Visa { number: 1024 };
    transact(card2); // Panic!
}
