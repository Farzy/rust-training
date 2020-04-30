use std::net::{IpAddr, Ipv4Addr};

pub struct Approval<T> {
    item: T,
    approved: bool,
}

impl<T> Approval<T> {
    pub fn new(item: T) -> Approval<T> {
        Approval {
            item,
            approved: false,
        }
    }

    pub fn approve(&mut self) {
        self.approved = true;
    }

    // pub fn replace<U>(self, other_item: U) -> Approval<U> {
    //     Approval {
    //         item: other_item,
    //         approved: self.approved,
    //     }
    // }

    pub fn replace(self, other_item: T) -> Approval<T> {
        Approval {
            item: other_item,
            approved: self.approved,
        }
    }

    pub fn approved_item(&self) -> Option<&T> {
        if self.approved {
            Some(&self.item)
        } else {
            None
        }
    }
}

pub fn main() {
    let amount = 1000;
    let mut approval_ammount = Approval::new(amount);
    assert!(approval_ammount.approved_item().is_none());
    approval_ammount.approve();
    assert_eq!(approval_ammount.approved_item(), Some(&1000));

    let mut approval_ip = Approval::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert!(approval_ip.approved_item().is_none());
    approval_ip.approve();
    assert_eq!(
        approval_ip.approved_item(),
        Some(&IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
    );

    // Non working code
    // let replaced = approval_ip.replace(3);

    let replaced = approval_ip.replace(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)));
    assert!(replaced.approved_item().is_some());
}
