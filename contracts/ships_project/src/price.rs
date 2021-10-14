use near_sdk::env;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::U128;
/*
    sigmoid curve pricing function
    In this case we shift the curve by 3 times the midpoint
    this allows us to have a longer s portion relative to the midpoint a
    less steep incline that's based off of the token midpoint
    a: curve_height
    b: curve_midpoint
    integral a (1 + (-b + x)/sqrt(3 b + (-b + x)^2)) dx = a (sqrt(b^2 - 2 b x + 3 b + x^2) + x) + constant
    \frac{a}{2}\left(\frac{x-b\ }{\sqrt{3b+\left(x-b\right)^{2}}}\ +1\right)\
    reference: https://www.desmos.com/calculator/bmikymuori
    reference: https://www.wolframalpha.com/input/?i=a*%28%28x-b%29%2Fsqrt%283b%2BPower%5B%28x-b%29%2C2%5D%29+%2B+1%29
    reference: https://blog.relevant.community/bonding-curves-in-depth-intuition-parametrization-d3905a681e0a
 */

/*
 This includes the marginal price percentage. The function is as follows
 the (max-min) * % of marginal range of value to occupy
 for (0.1,0.1,0.2,0.2,0.2)
 the result is that 10% of the difference is occupied by first tier
 followed by another 10% for second tier, followed by 20% for the
 third tier ... until 100% of the difference is defimed by the 
 cumulative value of the tier.

 This is done to provide a pricing guideline and range that makes 
 sense given a target of tokens to move, placing a cap on volume 
*/
pub const PRICING_TIER: &[f64; 6] = &[0.1, 0.1, 0.2, 0.2, 0.2, 0.2];

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct PricingCurve {
    pub(crate) min: U128,
    pub(crate) max: U128,
    pub(crate) token_cap: U128,
}

// Curve pricing must be adjusted later to fit the decimals application
// See Pricing Tier commnet
impl PricingCurve {
    /*
    let rng = (max-min);
    let break_prices =  bpts.map((x)=> x * rng + min)
    let break_tokens = bpts.map((x)=> x * max_tokens)
     */

    fn internal_balance_cost(&self, tokens: u128) -> f64 {
        let rng = (self.max.0 - self.min.0) as f64;
        let mut break_prices: [f64; PRICING_TIER.len()] =[0.0; PRICING_TIER.len()];
        let mut break_tokens: [f64; PRICING_TIER.len()] =[0.0; PRICING_TIER.len()];
        PRICING_TIER.iter().enumerate().for_each(|(i, pt)| {
            break_prices[i]=pt * rng;
            break_tokens[i]=pt * (self.token_cap.0 as f64);
        });
        let mut new_tokens: f64 = tokens as f64;
        let mut cost  = 0.0;
        let mut running_price = self.min.0 as f64;
        let mut remaining_tokens = 0.0;
        for i in 0..break_tokens.len(){
            remaining_tokens = new_tokens - break_tokens[i];
            if remaining_tokens <= 0.0 {
                cost += new_tokens * running_price;
                return  cost;
            }
            cost += break_tokens[i] * running_price;
            new_tokens = remaining_tokens;
            running_price += break_prices[i];
        }
        if remaining_tokens > 0.0 {
            cost += remaining_tokens * self.max.0 as f64;
        }
        cost
    }

    pub fn price(&self, tokens: u128, supply: u128) -> f64 {
        self.internal_balance_cost(tokens.checked_add(supply).unwrap())
            - self.internal_balance_cost(supply)
    }
}


#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basic_calculation() {
        let curve = PricingCurve {
            token_cap: 100.into(),
            max: 5.into(),
            min: 1.into()
        };
        assert_eq!(curve.price(0, 0 ), 0.0);
        assert_eq!(curve.price(1, 0 ), 1.0);
        //  pricing with existing supply is correct
        assert_eq!(curve.price(100, 0 ), 264.0);
        //  pricing with existing supply is correct
        assert_eq!(curve.price(50, 50 ), 178.0);
        // max token supply marginal pricing is correct
        assert!(curve.price(1, 99 ) < 5.0, "Must hold true");
        // max token supply holds true
        assert_eq!(curve.price(1, 100 ), 5.0);
    }
}
