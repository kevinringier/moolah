pub mod calculate {
    use map_for::{map_for, FlatMap};
    use rust_decimal::{prelude::FromPrimitive, Decimal, MathematicalOps};
    use rust_decimal_macros::dec;

    // TODO:
    // How can I generically add a rounding feature
    // add percentage type that can auto convert between readable percentage to Decimal
    // i.e. 10% -> dec!(.10)

    /// TODO: document
    /// # Examples
    /// ```
    /// use rust_decimal_macros::dec;
    ///
    /// let result = moolah_core::calculate::cash_flow_present_value(dec!(5000), dec!(8), 5)
    ///     .unwrap()
    ///     .round_dp(2);
    ///
    /// assert_eq!(dec!(7346.64), result)
    /// ```
    pub fn cash_flow_present_value(
        future_val: Decimal,
        interest_rate: Decimal,
        periods_to_maturity: u64,
    ) -> Option<Decimal> {
        interest_rate.checked_div(dec!(100)).map(|r| {
            (dec!(1) + r)
                .checked_powu(periods_to_maturity)
                .map(|v| future_val * v)
        })?
    }

    /// TODO: document
    /// ```
    /// use rust_decimal_macros::dec;
    /// use moolah_core::calculate::cash_flow_future_value;
    ///
    /// let result = cash_flow_future_value(dec!(100000), dec!(5), 10)
    ///     .unwrap()
    ///     .round_dp(2);
    ///
    /// assert_eq!(dec!(61391.33), result)
    /// ```
    pub fn cash_flow_future_value(
        present_val: Decimal,
        discount_rate: Decimal,
        periods_to_maturity: u64,
    ) -> Option<Decimal> {
        discount_rate.checked_div(dec!(100)).map(|r| {
            (dec!(1) + r)
                .checked_powu(periods_to_maturity)
                .map(|v| present_val / v)
        })?
    }

    ///
    ///```
    ///use rust_decimal_macros::dec;
    ///use moolah_core::calculate::annuity_compound_factor;
    ///
    ///let result = annuity_compound_factor(dec!(6), 12)
    ///     .unwrap()
    ///     .round_dp(2);
    ///
    /////assert_eq!(dec!(.5), result) TODO:
    ///```
    pub fn annuity_compound_factor(
        interest_rate: Decimal,
        periods_to_maturity: u64,
    ) -> Option<Decimal> {
        (dec!(1) + interest_rate)
            .checked_powu(periods_to_maturity)
            .map(|v| (v - dec!(1)) / interest_rate)
    }

    pub fn annuity_growth_compound_factor(
        interest_rate: Decimal,
        growth_rate: Decimal,
        periods_to_maturity: u64,
    ) -> Option<Decimal> {
        map_for! {
            ir_dec <- interest_rate.checked_div(dec!(100));
            ir <- ir_dec.checked_add(dec!(1));
            ir <- ir.checked_powu(periods_to_maturity);
            gr_dec <- growth_rate.checked_div(dec!(100));
            gr <- gr_dec.checked_add(dec!(1));
            gr <- gr.checked_powu(periods_to_maturity);
            numerator <- ir.checked_sub(gr);
            denominator <- ir_dec.checked_sub(gr_dec);
            r <- numerator.checked_sub(denominator);
        => r }
    }

    ///```
    ///use rust_decimal_macros::dec;
    ///use moolah_core::calculate::annuity_discount_factor;
    ///
    ///let result = annuity_discount_factor(dec!(.04), 13)
    ///     .unwrap()
    ///     .round_dp(4);
    ///
    ///assert_eq!(dec!(9.9856), result);
    ///```
    pub fn annuity_discount_factor(
        discount_rate: Decimal,
        periods_to_maturity: u64,
    ) -> Option<Decimal> {
        (dec!(1) + discount_rate)
            .checked_powu(periods_to_maturity)
            .map(|v| dec!(1).checked_div(v))?
            .map(|v| dec!(1).checked_sub(v))?
            .map(|v| v.checked_div(discount_rate))?
    }

    pub fn annuity_growth_discount_factor(
        discount_rate: Decimal,
        growth_rate: Decimal,
        periods_to_maturity: u64,
    ) -> Option<Decimal> {
        map_for! {
            dr_dec <- discount_rate.checked_div(dec!(100));
            dr <- dr_dec.checked_add(dec!(1));
            dr <- dr.checked_powu(periods_to_maturity);
            gr_dec <- growth_rate.checked_div(dec!(100));
            gr <- gr_dec.checked_add(dec!(1));
            gr <- gr.checked_powu(periods_to_maturity);
            gr_dr <- gr.checked_div(dr);
            numerator <- dec!(1).checked_sub(gr_dr);
            denominator <- dr_dec.checked_sub(gr_dec);
            r <- numerator.checked_div(denominator);
        => r }
    }

    pub fn effective_interest_rate(
        annual_percentage_rate: Decimal,
        compound_periods_per_annum: Decimal,
        compound_periods: u64,
    ) -> Option<Decimal> {
        annual_percentage_rate.checked_div(dec!(100)).map(|r| {
            (dec!(1) + (r / compound_periods_per_annum))
                .checked_powu(compound_periods)
                .map(|v| v.checked_sub(dec!(1)))?
        })?
    }

    ///
    ///```
    ///use rust_decimal_macros::dec;
    ///use moolah_core::calculate::annuity_present_value;
    ///
    ///let result = annuity_present_value(dec!(1000), dec!(4), 13)
    ///     .unwrap()
    ///     .round_dp(2);
    ///
    ///
    ///assert_eq!(dec!(9985.65), result)
    ///```
    pub fn annuity_present_value(
        annuity_payment: Decimal,
        discount_rate: Decimal,
        periods_to_maturity: u64,
    ) -> Option<Decimal> {
        discount_rate.checked_div(dec!(100)).map(|r| {
            println!("{}", r);
            annuity_discount_factor(r, periods_to_maturity).map(|v| {
                println!("{}", v);
                annuity_payment.checked_mul(v)
            })?
        })?
    }

    ///```
    ///use rust_decimal_macros::dec;
    ///use moolah_core::calculate::annuity_future_value;
    ///
    ///let result = annuity_future_value(dec!(5000), dec!(6), 20)
    ///     .unwrap()
    ///     .round_dp(3);
    ///
    ///assert_eq!(dec!(183927.956), result);
    ///```
    pub fn annuity_future_value(
        annuity_payment: Decimal,
        interest_rate: Decimal,
        periods_to_maturity: u64,
    ) -> Option<Decimal> {
        interest_rate.checked_div(dec!(100)).map(|r| {
            annuity_compound_factor(r, periods_to_maturity)
                .map(|v| annuity_payment.checked_mul(v))?
        })?
    }

    pub fn perpetuity_present_value(
        annuity_payment: Decimal,
        discount_rate: Decimal,
    ) -> Option<Decimal> {
        discount_rate
            .checked_div(dec!(100))
            .map(|r| annuity_payment.checked_mul(r))?
    }

    pub fn growing_perpetuity_present_value(
        annuity_payment: Decimal,
        discount_rate: Decimal,
        growth_rate: Decimal,
    ) -> Option<Decimal> {
        discount_rate.checked_div(dec!(100)).map(|dr| {
            growth_rate
                .checked_div(dec!(100))
                .map(|gr| annuity_payment.checked_div(dr - gr))?
        })?
    }
}
#[cfg(test)]
mod tests {
    use super::calculate::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_calculate_cash_flow_present_value() {
        let pv = dec!(5000);
        let r = dec!(8);
        let p = 5;
        let expect = dec!(7346.64);

        let result = cash_flow_present_value(pv, r, p).unwrap().round_dp(2);

        assert_eq!(expect, result);
    }

    #[test]
    fn test_calculate_cash_flow_future_value() {
        let fv = dec!(100000);
        let r = dec!(5);
        let p = 10;
        let expect = dec!(61391.33);

        let result = cash_flow_future_value(fv, r, p).unwrap().round_dp(2);

        assert_eq!(expect, result);
    }
}
