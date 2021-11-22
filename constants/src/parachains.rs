/// Karura chain specs.
pub mod karura {
    use primitives::currency::CurrencyId;
    use smallvec::smallvec;
    use sp_runtime::Perbill;
    use frame_support::weights::{
		constants::{ExtrinsicBaseWeight, WEIGHT_PER_SECOND},
		WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
	};

    /// Karura chain id.
    pub const CHAIN_ID: u32 = 2000;

    /// Karura balance type.
    pub type KaruraBalance = u128;

    /// Get 1 dollar of currency.
    pub fn dollar(currency_id: &CurrencyId) -> KaruraBalance {
        10u128.saturating_pow(currency_id.decimals() as u32)
    }

    /// Get 1 cent of currency. 
    pub fn cent(currency_id: &CurrencyId) -> KaruraBalance {
        dollar(currency_id) / 100
    }

    /// Copy from Acala runtime.
    fn base_tx_in_kar() -> KaruraBalance {
		cent(&CurrencyId::KAR) / 10
	}

    pub struct WeightToFee;
	impl WeightToFeePolynomial for WeightToFee {
		type Balance = KaruraBalance;

		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			// in Karura, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
			let p = base_tx_in_kar();
			let q = Self::Balance::from(ExtrinsicBaseWeight::get());

			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q),
				coeff_integer: p / q,
			}]
		}
	}

	pub fn kar_per_second() -> u128 {
		let base_weight = KaruraBalance::from(ExtrinsicBaseWeight::get());
		let base_tx_per_second = (WEIGHT_PER_SECOND as u128) / base_weight;
		base_tx_per_second * base_tx_in_kar()
	}

	pub fn ksm_per_second() -> u128 {
		kar_per_second() / 50
	}
}
