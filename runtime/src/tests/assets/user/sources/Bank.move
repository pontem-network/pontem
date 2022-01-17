module RuntimeTests::Bank {
    use Std::Signer;
    use PontemFramework::Token::{Self, Token};

    struct Storage<phantom TokenType> has key, store {
        balance: Token<TokenType>,
    }

    public fun deposit<TokenType>(account: &signer, deposit: Token<TokenType>) acquires Storage {
        let addr = Signer::address_of(account);

        if (!exists<Storage<TokenType>>(addr)) {
            move_to(account, Storage {
                balance: deposit
            });
        } else {
            let storage = borrow_global_mut<Storage<TokenType>>(addr);
            Token::deposit<TokenType>(&mut storage.balance, deposit);
        }
    }
}
