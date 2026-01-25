use crate::models::shop::{
    FreeleechTokenDiscountTier, FreeleechTokensPriceCalculation, UploadDiscountTier,
    UploadPriceCalculation,
};

const BYTES_PER_GB: i64 = 1_073_741_824;

pub fn calculate_upload_price(
    bytes: i64,
    base_price_per_gb: i64,
    discount_tiers: &[UploadDiscountTier],
) -> UploadPriceCalculation {
    let gb = bytes / BYTES_PER_GB;
    let base_price = gb * base_price_per_gb;

    // Find the highest applicable discount tier
    let discount_percent = discount_tiers
        .iter()
        .filter(|tier| gb >= tier.threshold_gb)
        .map(|tier| tier.discount_percent)
        .max()
        .unwrap_or(0);

    let discount_multiplier = 100 - discount_percent as i64;
    let final_price = (base_price * discount_multiplier) / 100;

    UploadPriceCalculation {
        bytes,
        base_price,
        discount_percent,
        final_price,
    }
}

pub fn calculate_freeleech_tokens_price(
    quantity: i32,
    base_price_per_token: i64,
    discount_tiers: &[FreeleechTokenDiscountTier],
) -> FreeleechTokensPriceCalculation {
    let base_price = quantity as i64 * base_price_per_token;

    // Find the highest applicable discount tier
    let discount_percent = discount_tiers
        .iter()
        .filter(|tier| quantity >= tier.threshold)
        .map(|tier| tier.discount_percent)
        .max()
        .unwrap_or(0);

    let discount_multiplier = 100 - discount_percent as i64;
    let final_price = (base_price * discount_multiplier) / 100;

    FreeleechTokensPriceCalculation {
        quantity,
        base_price,
        discount_percent,
        final_price,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_upload_price_no_discount() {
        let tiers = vec![
            UploadDiscountTier {
                threshold_gb: 10,
                discount_percent: 10,
            },
            UploadDiscountTier {
                threshold_gb: 50,
                discount_percent: 20,
            },
        ];

        let result = calculate_upload_price(5 * BYTES_PER_GB, 100, &tiers);
        assert_eq!(result.base_price, 500);
        assert_eq!(result.discount_percent, 0);
        assert_eq!(result.final_price, 500);
    }

    #[test]
    fn test_calculate_upload_price_with_discount() {
        let tiers = vec![
            UploadDiscountTier {
                threshold_gb: 10,
                discount_percent: 10,
            },
            UploadDiscountTier {
                threshold_gb: 50,
                discount_percent: 20,
            },
        ];

        let result = calculate_upload_price(60 * BYTES_PER_GB, 100, &tiers);
        assert_eq!(result.base_price, 6000);
        assert_eq!(result.discount_percent, 20);
        assert_eq!(result.final_price, 4800);
    }

    #[test]
    fn test_calculate_freeleech_tokens_price_no_discount() {
        let tiers = vec![
            FreeleechTokenDiscountTier {
                threshold: 5,
                discount_percent: 10,
            },
            FreeleechTokenDiscountTier {
                threshold: 10,
                discount_percent: 15,
            },
        ];

        let result = calculate_freeleech_tokens_price(3, 500, &tiers);
        assert_eq!(result.base_price, 1500);
        assert_eq!(result.discount_percent, 0);
        assert_eq!(result.final_price, 1500);
    }

    #[test]
    fn test_calculate_freeleech_tokens_price_with_discount() {
        let tiers = vec![
            FreeleechTokenDiscountTier {
                threshold: 5,
                discount_percent: 10,
            },
            FreeleechTokenDiscountTier {
                threshold: 10,
                discount_percent: 15,
            },
        ];

        let result = calculate_freeleech_tokens_price(10, 500, &tiers);
        assert_eq!(result.base_price, 5000);
        assert_eq!(result.discount_percent, 15);
        assert_eq!(result.final_price, 4250);
    }
}
