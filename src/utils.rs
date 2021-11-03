use optioncontracts::OptionContract;

// we have 8 args but clippy wants 7 max
#[allow(clippy::too_many_arguments)]
pub fn find_intersection(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
    x4: f64,
    y4: f64,
) -> (f64, f64) {
    let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4))
        / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));
    let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4))
        / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));
    (px, py)
}

pub fn find_zeros(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
    // find x where y equals zero
    // derived from find_intersection where y3 and y4 are 0
    let px = ((x1 * y2 - y1 * x2) * (x2 - x1)) / (0.0 - (y1 - y2) * (x2 - x1));
    (px, 0.0)
}

pub fn get_zero_intersections(
    lines_through_zero: Vec<((f64, f64), (f64, f64))>,
) -> Vec<(f64, f64)> {
    let mut zero_intersections = vec![];

    // iterate over vector of tuples of coordinates
    for (first, second) in lines_through_zero.iter() {
        let intersection = find_zeros(
            first.0,  // x1: f64,
            first.1,  // y1: f64,
            second.0, // x2: f64,
            second.1, // y2: f64,
        );
        zero_intersections.push(intersection);
    }
    zero_intersections
}

pub fn get_lines_through_zero(v: Vec<(f64, f64)>) -> Vec<((f64, f64), (f64, f64))> {
    // init a vector to populate in iterations
    // theres probably a way to inline this... one day I can fix that
    let mut zero_line_breaks: Vec<((f64, f64), (f64, f64))> = vec![];
    // iterate over 2 items at a time ie: [0,1], [1,2], [2,3], ...
    v.windows(2)
        .inspect(|w| {
            let left = w[0].1;
            let right = w[1].1;

            let min = left.min(right);
            let max = left.max(right);

            if min <= 0.0 && max > 0.0 {
                zero_line_breaks.push((w[0], w[1]))
            }
        })
        .for_each(drop);
    zero_line_breaks
}

pub fn contract_intrinsic_values_at_price(
    current_price: f64,
    contracts: Vec<OptionContract>,
) -> f64 {
    // apply execute_option for all contracts and sum result
    contracts
        .iter()
        .map(|contract| optioncontracts::execute_option(&contract, current_price))
        .sum()
}

pub fn calculate_contract_values(contracts: Vec<OptionContract>) -> Vec<(f64, f64)> {
    // options must be sorted
    // this gets all unique prices
    let mut prices: Vec<f64> = vec![];
    let mut last = 0.0;

    prices.push(0.0); // for lower end
    for s in contracts.iter() {
        if s.strike - last < 0.00001 {
            continue;
        }
        last = s.strike;
        prices.push(s.strike)
    }
    // prices.push(f64::MAX);
    prices.push(1000.0);

    // for each price we find the y (p/l) value
    // this generates (x,y) coords of the payout diagram
    let v: Vec<(f64, f64)> = prices
        .iter()
        .map(|p| {
            // builds: (X, Y)
            (
                *p,
                contracts
                    .iter()
                    .map(|contract| optioncontracts::execute_option(&contract, *p))
                    .sum(),
            )
        })
        .collect();
    v
}
