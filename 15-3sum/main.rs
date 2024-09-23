impl Solution {
    pub fn three_sum(nums: Nums) -> ResultValue {
        Nums::is_valid(nums).unwrap_or_else(|err| {
            match err {
                "Triple zeros" => vec![vec![0,0,0]],
                "Cannot create zero" => vec![],
                _ => vec![],
            }
        })
    }
}

type Nums = Vec<i32>;
type ResultValue = Vec<Vec<i32>>;
type AtomValue = i32;

impl Checker for Nums {
    fn is_valid(nums: Nums) -> Result<ResultValue, &'static str> {
        match nums[..] {
            [0, 0, 0] => Err("Triple zeros"),
            [0, 1, 1] => Err("Cannot create zero"),
            _ => Ok(Self::calc(nums)),
        }
    }

    fn calc(nums: Nums) -> ResultValue {
        let result: ResultValue = vec![vec![]];
        for (i, first_value) in nums.iter().enumerate() {

        }
        result
    }
}

trait Checker {
    fn is_valid(nums: Nums) -> Result<ResultValue, &'static str>;
    fn calc(nums: Nums) -> ResultValue;
}
