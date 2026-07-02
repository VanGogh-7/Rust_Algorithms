fn build_array(target: Vec<i32>) -> Vec<String> {
    let mut result = Vec::with_capacity(target.len() * 2);
    let mut curr = 1;

    for &t in &target {
        while curr < t {
            result.push("push".to_string());
            result.push("pop".to_string());
            curr = curr + 1;
        }
        result.push("push".to_string());
        curr += 1;
    }
    result
}

fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::with_capacity(tokens.len() / 2 + 1);

    for token in tokens {
        match token.as_str() {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }
            "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b);
            }
            _ => {
                let num = token.parse::<i32>().unwrap();
                stack.push(num);
            }
        }
    }
    stack.pop().unwrap()
}

fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let mut result = vec![0; n as usize];

    let mut stack = Vec::with_capacity(logs.len() / 2);
    let mut prev_time = 0;

    for log in &logs {
        let mut parts = log.split(":");
        let id = parts.next().unwrap().parse::<usize>().unwrap();
        let is_start = parts.next().unwrap() == "start";
        let timestamp = parts.next().unwrap().parse::<i32>().unwrap();

        if is_start {
            if let Some(&top_id) = stack.last() {
                result[top_id] += timestamp - prev_time;
            }
            stack.push(id);
            prev_time = timestamp;
        } else {
            let top_id = stack.pop().unwrap();
            result[top_id] += timestamp - prev_time + 1;
            prev_time = timestamp + 1;
        }
    }
    result
}
fn main() {
    let target: Vec<i32> = vec![1, 3];
    let result = build_array(target);
    println!("result: {:?}", result);

    let tokens: Vec<String> = vec![
        "2".to_string(),
        "1".to_string(),
        "+".to_string(),
        "3".to_string(),
        "*".to_string(),
    ];
    let result = eval_rpn(tokens);
    println!("result: {:?}", result);

    let logs: Vec<String> = vec!["0:start:0".to_string(), "1:start:2".to_string(), "2:end:5".to_string(), "0:end:6".to_string()];
    let result = exclusive_time(2, logs);
    println!("result: {:?}", result);

}
