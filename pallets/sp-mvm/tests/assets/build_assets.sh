dove clean
dove build --tree
dove ct "store_u64(42)"
dove ct "emit_event(42)"
dove ct "store_system_block()"
dove ct "store_system_timestamp()"
dove ct "inf_loop()"
dove ct "store_native_balance()"
dove ct "store_native_deposit(false)"
dove ct "store_native_deposit(true)" -o=store_native_deposit_reg
dove ct "store_native_withdraw(false)"
dove ct "store_native_withdraw(true)" -o=store_native_withdraw_reg
dove ct "test_balance_transfer(5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY, 42, true)"
