address {{sender}} {
module Bank {
    use 0x1::Signer;
    use 0x1::Diem::{Self, Diem};

    struct Storage<Token: key + store> has key, store {
        balance: Diem<Token>,
    }

    public fun deposit<Token: key + store>(account: &signer, deposit: Diem<Token>) acquires Storage {
        let addr = Signer::address_of(account);

        if (!exists<Storage<Token>>(addr)) {
            move_to(account, Storage {
                balance: deposit
            });
        } else {
            let storage = borrow_global_mut<Storage<Token>>(addr);
            Diem::deposit<Token>(&mut storage.balance, deposit);
        }
    }
}
}
