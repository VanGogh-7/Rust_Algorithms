fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut counts = [0; 2];

    for &pref in &students {
        counts[pref as usize] += 1;
    }

    for &sandwich in &sandwiches {
        let type_idx = sandwich as usize;

        if counts[type_idx] > 0 {
            counts[type_idx] -= 1;
        } else {
            break;
        }
    }
    (counts[0] + counts[1]) as i32
}

fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let target = tickets[k];
    tickets
        .iter()
        .enumerate()
        .map(|(i, &ticket)| {
            if i <= k {
                ticket.min(target)
            } else {
                ticket.min(target - 1)
            }
        })
        .sum()
}

struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }
    fn push(&mut self, x: i32) {
        self.in_stack.push(x);
    }
    fn pop(&mut self) -> i32 {
        self.move_in_to_out();
        self.out_stack.pop().unwrap()
    }
    fn peek(&mut self) -> i32 {
        self.move_in_to_out();
        self.out_stack.pop().unwrap()
    }

    fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }
    fn move_in_to_out(&mut self) {
        if self.out_stack.is_empty() {
            while let Some(x) = self.in_stack.pop() {
                self.out_stack.push(x);
            }
        }
    }
}
fn main() {
    let students = vec![1, 1, 0, 0];
    let sandwiches = vec![0, 1, 0, 1];
    let result = count_students(students, sandwiches);
    println!("{}", result);

    let tickets = vec![2,3,2];
    let k = 2;
    let result = time_required_to_buy(tickets, k);
    println!("{}", result);

    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.peek();
    queue.pop();
    queue.push(4);
    queue.pop();
    queue.pop();
    queue.pop();
    queue.empty();



}
