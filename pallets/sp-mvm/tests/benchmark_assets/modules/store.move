address 0x1 {
    module Store {
        const BUFF_1KB: vector<u8> = x"15deb2509eefc8093a8b24465d73a078b4e1a3c7ded8f46a296277d6e8ce7fadcb63ad4127492cdc296d81d98c1f0d4e0da38f047160a8b3b6cc2bda4557f547f7ea87081011583dc37a36785f292e6322789bcf7b9957c50cd6b8f3b028ff5efacda8e56da78f5ef2b285eddbfa7c41a6fa03103fa63aab1b94b5bb50aee70632a71e5965491b721385058d59ee8399e65dc1f821a02749e4ba21974c6fd5b84af603aa82e49eaa47e98e4d3609a7511480ac06e137ab431b53060bcac27dcc7f964d19d8f609f3a5fab1c8303272adfc453912bfeb61c8d46d3e63a18b0ac858a5e7a77dacf1f12f3aa5af272db104a96520c9df09957ae9a4480b73fe1108a5e265c4f5c4130778c3f2a378c5177bd4b8dfb292772578beeed5c3e23e05f9167aa054e9f1bc9828235e3a4f25f954f51a3ca0f66e611d30679fb2fc4e0300b75488ae993cf9db2be9ce4b00330713e95eac211d4deb7ec711fd3017e03f3a6025afe6536228c6e4a517a600b6818d236596ead62a11bceab3d5efc7c835f97627a125e9ef9a7ea4afeb1a16b0fe669ab93a6e8234ae8b64362d7194016889d104de46d155154f09e873331b491e24962af9ba838906e82ad09d0328361bd600d20dd0d98ec6c27cd07b6b76497bfb1ca1b84a512739f415af7c534c9c7fe23b09c53a13f0e7d1738deaa20a8bf9dff21d775ef8fa2866c4c6b422b495c7a3b5a24e876141d4c7fca791b70f8ffe5b2075687fa76c2883f13b875c61f210de963805f00e6e1ed46216a4563859e4689e04736ed8b409a51c77e8a9fa48a0e324b8b3888fbffdaf9374bf039d3024bd3a3241481457650b8f806d9b393f26646d66bc47f9c7c9d6776d886854503d154c7a80118ea973524a432b8dd86d868dec4eb93fac075982388bc9558150776e6013c500863a1ac641a3543cb1dcc717a9ba59fac7989173a5f5d666ae4141fabe9b2f33f851c7e15c0c566cc8482fb4133cc29b2dcb80da207870c894dea916df821af4749efe4f7e961f4e9aa800227d23ae5940a305bd60b958db132e3ae89ddfe53b8fc00f685839d6387bed65ff519052e0b84006dacc003b59c257b7f473367564c792a6195e5b301ec5b60406131bc90a06341316cf898e442c98bffb382e61b1c915644d0d4f8b9f0240f03fdf4da6fd2b7e54a684b02acd4dd7e7a98f15b87c0bb78a411b42f2118e1b7b771e30561c7659a62c917360b48ccbce4bb57b062bb806f72e26abd2248f2893bcf1f7b330971582a027e8c35a13be5756e47852d9f060336d7ce14d01642396cdead66306b0b074a30805136e065a671c2cbf9d05abdebacc31922e9e0e27a75acd3bbcc7e7e1f13f36af28e9e4f91d25b42a2ec736ad8f5f12bdeacd3cf8984dd70ef981ed11309942fb10c1cfcd1dce97322e0bd0a6d2293b77adfbe904c905";

        resource struct Container {
            inner_1: Inner,
            inner_2: Inner,
            inner_3: Inner,
            inner_4: Inner,
        }

        resource struct Inner {
            val: bool,
            val_1: u128,
            val_2: vector<u8>,
            val_3: u64,
        }

        public fun store(s: &signer) {
            let container = Container {
                inner_1: Inner {
                    val: true,
                    val_1: 1000000000,
                    val_2: BUFF_1KB,
                    val_3: 13,
                },
                inner_2: Inner {
                    val: false,
                    val_1: 13,
                    val_2: BUFF_1KB,
                    val_3: 0,
                },
                inner_3: Inner {
                    val: false,
                    val_1: 42,
                    val_2: BUFF_1KB,
                    val_3: 0,
                },
                inner_4: Inner {
                    val: false,
                    val_1: 0,
                    val_2: BUFF_1KB,
                    val_3: 0,
                },
            };
            move_to<Container>(s, container);
        }

        public fun exisit(): bool {
            exists<Container>(0x1)
        }

        public fun borrow() acquires Container {
            let _c = borrow_global_mut<Container>(0x1);
        }

    }
}