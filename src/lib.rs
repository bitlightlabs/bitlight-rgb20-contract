#[cfg(test)]
mod tests {
    use amplify::hex::FromHex;
    use bp::dbc::Method;
    use bp::{Outpoint, Txid};
    use ifaces::Rgb20;
    use rgbstd::containers::{ConsignmentExt, FileContent};
    use rgbstd::interface::{FilterIncludeAll, FungibleAllocation};
    use rgbstd::invoice::Precision;
    use rgbstd::persistence::Stock;
    use rgbstd::XWitnessId;
    use schemata::dumb::NoResolver;
    use schemata::NonInflatableAsset;

    #[test]
    fn test_contract_creation() {
        let beneficiary_txid =
            Txid::from_hex("14295d5bb1a191cdb6286dc0944df938421e3dfcbf0811353ccac4100c2068c5")
                .unwrap();
        let beneficiary = Outpoint::new(beneficiary_txid, 1);

        #[allow(clippy::inconsistent_digit_grouping)]
        let contract = NonInflatableAsset::testnet(
            "ssi:anonymous",
            "TEST",
            "Test asset",
            None,
            Precision::CentiMicro,
            [(Method::TapretFirst, beneficiary, 1_000_000_000_00u64)],
        )
        .expect("invalid contract data");

        let contract_id = contract.contract_id();

        eprintln!("{contract}");
        contract
            .save_file("test/rgb20-simplest.rgb")
            .expect("unable to save contract");
        contract
            .save_armored("test/rgb20-simplest.rgba")
            .expect("unable to save armored contract");

        // Let's create some stock - an in-memory stash and inventory around it:
        let mut stock = Stock::in_memory();
        stock.import_contract(contract, NoResolver).unwrap();

        // Reading contract state through the interface from the stock:
        let contract = stock.contract_iface_class::<Rgb20>(contract_id).unwrap();
        let allocations = contract.allocations(&FilterIncludeAll);

        for FungibleAllocation {
            seal,
            state,
            witness,
            ..
        } in allocations
        {
            let witness = witness
                .as_ref()
                .map(XWitnessId::to_string)
                .unwrap_or("~".to_owned());
            assert_eq!("adMhBHaQ", state.to_string());
            assert_eq!(
                "bc:tapret1st:14295d5bb1a191cdb6286dc0944df938421e3dfcbf0811353ccac4100c2068c5:1",
                seal.to_string()
            );
            assert_eq!("~", witness);
        }
    }
}
