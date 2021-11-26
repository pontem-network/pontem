/// Test parachains locations.
use crate::tests::mock::*;
use sp_runtime::traits::Convert;

#[test]
fn test_currency_id_convertations() {
    RuntimeBuilder::new().build().execute_with(|| {
        assert_eq!(RelayLocation::get(), MultiLocation::parent(),);

        assert_eq!(RelayNetwork::get(), NetworkId::Kusama);

        assert_eq!(
            CurrencyIdConvert::convert(CurrencyId::KSM),
            Some(MultiLocation::parent())
        );

        assert_eq!(
            CurrencyIdConvert::convert(CurrencyId::PONT),
            Some(MultiLocation {
                parents: 1,
                interior: X2(
                    Parachain(ParachainInfo::get().into()),
                    GeneralKey(CurrencyId::PONT.symbol())
                )
            })
        );

        assert_eq!(
            CurrencyIdConvert::convert(MultiLocation::parent()),
            Some(CurrencyId::KSM)
        );

        assert_eq!(
            CurrencyIdConvert::convert(MultiLocation {
                parents: 1,
                interior: X2(
                    Parachain(ParachainInfo::get().into()),
                    GeneralKey(CurrencyId::PONT.symbol()),
                )
            }),
            Some(CurrencyId::PONT),
        );
    });
}
