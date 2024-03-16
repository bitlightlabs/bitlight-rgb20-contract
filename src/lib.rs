#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::fs;
    use std::path::Path;

    use amplify::hex::FromHex;
    use armor::AsciiArmor;
    use bp::dbc::Method;
    use bp::{Outpoint, Txid};
    use rgb_schemata::NonInflatableAsset;
    use rgbstd::containers::FileContent;
    // use rgbstd::interface::{FilterIncludeAll, FungibleAllocation, IfaceClass, IssuerClass, Rgb20};
    use rgbstd::invoice::Precision;
    // use rgbstd::persistence::Inventory;
    use rgbstd::resolvers::ResolveHeight;
    use rgbstd::validation::{ResolveWitness, WitnessResolverError};
    use rgbstd::{WitnessAnchor, WitnessId, XAnchor, XPubWitness};
    use strict_encoding::StrictDumb;

    struct DumbResolver;

    impl ResolveWitness for DumbResolver {
        fn resolve_pub_witness(&self, _: WitnessId) -> Result<XPubWitness, WitnessResolverError> {
            Ok(XPubWitness::strict_dumb())
        }
    }

    impl ResolveHeight for DumbResolver {
        type Error = Infallible;
        fn resolve_anchor(&mut self, _: &XAnchor) -> Result<WitnessAnchor, Self::Error> {
            Ok(WitnessAnchor::strict_dumb())
        }
    }

    #[test]
    fn test_contract_creation() {
        let beneficiary_txid =
            Txid::from_hex("d6afd1233f2c3a7228ae2f07d64b2091db0d66f2e8ef169cf01217617f51b8fb")
                .unwrap();
        let beneficiary = Outpoint::new(beneficiary_txid, 1);

        let contract =
            NonInflatableAsset::testnet("TEST", "Test asset", None, Precision::CentiMicro)
                .expect("invalid contract data")
                .allocate(Method::TapretFirst, beneficiary, 100_000_000_000_u64.into())
                .expect("invalid allocations")
                .issue_contract()
                .expect("invalid contract data");

        let contract_id = contract.contract_id();
        eprintln!("contract_id {:?}", contract_id.to_string());
        assert!(contract_id.to_string().starts_with("rgb:"));

        eprintln!("{contract}");

        let contract_bin = "examples/rgb20-bit.unittest.contract.rgb";
        let contract_file = "examples/rgb20-bit.unittest.contract.rgba";

        contract
            .save_file(contract_bin)
            .expect("unable to save contract");
        fs::write(contract_file, contract.to_ascii_armored_string())
            .expect("unable to save contract");

        assert!(Path::new(contract_bin).exists());
        assert!(Path::new(contract_file).exists());
    }
}
