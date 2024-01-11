use cardano_serialization_lib::fees::LinearFee;
use cardano_serialization_lib::plutus as csl;
use cardano_serialization_lib::plutus::{ConstrPlutusData, ExUnitPrices, ExUnits};
use cardano_serialization_lib::tx_builder::{TransactionBuilder, TransactionBuilderConfigBuilder};
use cardano_serialization_lib::utils::to_bignum;
use cardano_serialization_lib::UnitInterval;
use plutus_ledger_api::plutus_data as pla;

pub(crate) fn create_tx_builder() -> TransactionBuilder {
    let linear_fee = &LinearFee::new(&to_bignum(44), &to_bignum(155381));

    let cfg = TransactionBuilderConfigBuilder::new()
        .fee_algo(linear_fee)
        .pool_deposit(&to_bignum(500000000))
        .key_deposit(&to_bignum(2000000))
        .max_value_size(4000)
        .max_tx_size(8000)
        .coins_per_utxo_byte(&to_bignum(34_482))
        .ex_unit_prices(&ExUnitPrices::new(
            &UnitInterval::new(&to_bignum(577), &to_bignum(10000)),
            &UnitInterval::new(&to_bignum(721), &to_bignum(10000000)),
        ))
        .build()
        .unwrap();
    TransactionBuilder::new(&cfg)
}

pub(crate) fn convert_plutus_data(pla_plutus_data: pla::PlutusData) -> csl::PlutusData {
    match pla_plutus_data {
        pla::PlutusData::Constr(i, d) => {
            csl::PlutusData::new_constr_plutus_data(&ConstrPlutusData::new(
                &to_bignum(u64::try_from(i).unwrap()),
                &csl::PlutusList::from(d.into_iter().map(convert_plutus_data).collect::<Vec<_>>()),
            ))
        }

        pla::PlutusData::Map(m) => {
            let mut pmap = csl::PlutusMap::new();

            m.into_iter().for_each(|(k, v)| {
                pmap.insert(&convert_plutus_data(k), &convert_plutus_data(v));
            });
            csl::PlutusData::new_map(&pmap)
        }
        pla::PlutusData::List(l) => csl::PlutusData::new_list(&csl::PlutusList::from(
            l.into_iter().map(convert_plutus_data).collect::<Vec<_>>(),
        )),
        pla::PlutusData::Integer(i) => {
            csl::PlutusData::new_integer(&cardano_serialization_lib::utils::BigInt::from(i))
        }
        pla::PlutusData::Bytes(b) => csl::PlutusData::new_bytes(b),
    }
}

pub(crate) fn to_redeemer(plutus_data: csl::PlutusData) -> csl::Redeemer {
    csl::Redeemer::new(
        &csl::RedeemerTag::new_spend(),
        &to_bignum(0),
        &plutus_data,
        &ExUnits::new(&to_bignum(1), &to_bignum(2)),
    )
}
