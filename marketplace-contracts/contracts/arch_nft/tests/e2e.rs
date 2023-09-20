extern crate arch_nft;

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    use archisinal_lib::traits::collection::collection_external::Collection;
    use ink_e2e::build_message;
    use ink_e2e::PolkadotConfig;
    use openbrush::contracts::psp34::extensions::mintable::psp34mintable_external::PSP34Mintable;
    use openbrush::contracts::psp34::psp34_external::PSP34;
    use openbrush::contracts::psp34::Id;
    use openbrush::traits::String;
    use test_helpers::address_of;

    use crate::arch_nft::ContractRef;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn test_constructor(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let royalty = 10;
        let token_name = String::from("my_psp34");
        let token_uri = String::from("ipfs//my_psp34");
        let additional_info = String::from("my_psp34 contract");

        // When
        let constructor = ContractRef::new(
            royalty,
            Some(token_name.clone()),
            Some(token_uri.clone()),
            Some(additional_info.clone()),
        );

        let address = client
            .instantiate("arch_nft", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        // Then
        let c_token_name = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.collection_name());
            client
                .call(&ink_e2e::alice(), _msg, 0, None)
                .await
                .expect("call failed")
        }
        .return_value();

        let c_token_uri = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.collection_uri());
            client
                .call(&ink_e2e::alice(), _msg, 0, None)
                .await
                .expect("call failed")
        }
        .return_value();

        let c_token_royalty = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.collection_royalty());
            client
                .call(&ink_e2e::alice(), _msg, 0, None)
                .await
                .expect("call failed")
        }
        .return_value();

        let c_token_additional_info = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.collection_additional_info());
            client
                .call(&ink_e2e::alice(), _msg, 0, None)
                .await
                .expect("call failed")
        }
        .return_value();

        assert_eq!(c_token_additional_info, Some(additional_info));
        assert_eq!(c_token_name, Some(token_name));
        assert_eq!(c_token_uri, Some(token_uri));
        assert_eq!(c_token_royalty, royalty);

        Ok(())
    }

    #[ink_e2e::test]
    async fn mint_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let royalty = 10;
        let token_name = String::from("my_psp34");
        let token_uri = String::from("ipfs//my_psp34");
        let additional_info = String::from("my_psp34 contract");

        let token_id = Id::U8(1);

        // When
        let constructor = ContractRef::new(
            royalty,
            Some(token_name.clone()),
            Some(token_uri.clone()),
            Some(additional_info.clone()),
        );

        let address = client
            .instantiate("arch_nft", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        let mint_msg = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.mint(address_of!(alice), token_id.clone()));
            client
                .call(&ink_e2e::alice(), _msg, 0, None)
                .await
                .expect("call failed")
        }
        .return_value();

        assert!(mint_msg.is_ok());

        Ok(())
    }

    #[ink_e2e::test]
    async fn transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let royalty = 10;
        let token_name = String::from("my_psp34");
        let token_uri = String::from("ipfs//my_psp34");
        let additional_info = String::from("my_psp34 contract");

        let token_id = Id::U8(1);

        // When
        let constructor = ContractRef::new(
            royalty,
            Some(token_name.clone()),
            Some(token_uri.clone()),
            Some(additional_info.clone()),
        );

        let address = client
            .instantiate("arch_nft", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        let mint_msg = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.mint(address_of!(alice), token_id.clone()));
            client
                .call(&ink_e2e::alice(), _msg, 0, None)
                .await
                .expect("call failed")
        }
        .return_value();

        assert!(mint_msg.is_ok());

        let transfer_msg = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.transfer(address_of!(bob), token_id.clone(), vec![]));
            client
                .call(&ink_e2e::alice(), _msg, 0, None)
                .await
                .expect("call failed")
        };

        assert!(transfer_msg.return_value().is_ok());

        Ok(())
    }

    #[ink_e2e::test]
    async fn cant_change_token_uri_if_not_owner(
        mut client: ink_e2e::Client<C, E>,
    ) -> E2EResult<()> {
        // Given
        let royalty = 10;
        let token_name = String::from("my_psp34");
        let token_uri = String::from("ipfs//my_psp34");
        let additional_info = String::from("my_psp34 contract");

        // When
        let constructor = ContractRef::new(
            royalty,
            Some(token_name.clone()),
            Some(token_uri.clone()),
            Some(additional_info.clone()),
        );

        let address = client
            .instantiate("arch_nft", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        let set_collection_uri_msg = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.set_collection_uri(String::from("new_uri")));
            client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
        }
        .return_value();

        assert!(matches!(set_collection_uri_msg, Err(_)));

        Ok(())
    }
}
