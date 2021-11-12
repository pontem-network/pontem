address {{sender}} {
module BankPONT {
    use 0x1::Signer;
    use 0x1::Diem::{Self, Diem};
    use 0x1::PONT::PONT;

    struct Storage has key {
        balance: Diem<PONT>,
    }

    public fun deposit(account: &signer, deposit: Diem<PONT>) acquires Storage {
        let addr = Signer::address_of(account);

        if (!exists<Storage>(addr)) {
            move_to(account, Storage {
                balance: deposit
            });
        } else {
            let storage = borrow_global_mut<Storage>(addr);
            Diem::deposit<PONT>(&mut storage.balance, deposit);
        }
    }
}
}
