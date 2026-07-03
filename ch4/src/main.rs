fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::with_capacity(prices.len());
    for i in 0..prices.len() {
        while let Some(&last_idx) = stack.last() {
            if prices[i] <= prices[last_idx] {
                prices[last_idx] -= prices[i];
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    prices
}

fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();

    let mut ans = vec![0; n];
    let mut stack = Vec::with_capacity(n);

    for i in 0..n {
        while let Some(&last_idx) = stack.last() {
            if temperatures[i] > temperatures[last_idx] {
                ans[last_idx] = (i - last_idx) as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    ans
}

fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut max_area = 0;

    let mut stack = Vec::with_capacity(n);

    for i in 0..n {
        let cur_height = if i == n { 0 } else { heights[i] };

        while let Some(&last_idx) = stack.last() {
            if cur_height < heights[last_idx] {
                stack.pop();
                let h = heights[last_idx];

                let w = match stack.last() {
                    Some(&prev_idx) => (i - prev_idx - 1) as i32,
                    None => i as i32,
                };
                max_area = max_area.max(h * w);
            } else {
                break;
            }
        }
        stack.push(i);
    }
    max_area
}

fn main() {
    let prices = vec![8, 4, 6, 2, 3];
    let result = final_prices(prices);
    println!("result: {:?}", result);

    let temperatures = vec![73, 74, 75, 69, 72, 76, 73];
    let result = daily_temperatures(temperatures);
    println!("result: {:?}", result);

    let heights = vec![2, 1, 5, 6, 2, 3];
    let result = largest_rectangle_area(heights);
    println!("result: {:?}", result);
}
