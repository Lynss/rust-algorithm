#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let produce_stack = move |l: Option<Box<ListNode>>| {
        let mut result = vec![];
        let mut temp = l;
        while let Some(box ListNode { val, next }) = temp {
            result.push(val);
            temp = next;
        }
        result
    };
    let mut l1 = produce_stack(l1);
    let mut l2 = produce_stack(l2);
    let mut result = None;
    let mut up = 0;
    loop {
        let t1 = l1.pop();
        let t2 = l2.pop();
        if t1.is_none() && t2.is_none() && up == 0 {
            break;
        }
        let t1 = t1.unwrap_or(0);
        let t2 = t2.unwrap_or(0);
        let mut current = t1 + t2 + up;
        if current >= 10 {
            up = 1;
            current -= 10
        } else {
            up = 0;
        }
        result = Some(Box::new(ListNode {
            next: result.clone(),
            val: current,
        }));
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    //     Input: (7 -> 2 -> 4 -> 3) + (5 -> 6 -> 4)
    //     Output: 7 -> 8 -> 0 -> 7
    fn test_add_two_numbers() {
        let a = ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        };
        let b = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        };
        let c = ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 7, next: None })),
                })),
            })),
        };
        assert_eq!(
            c,
            *add_two_numbers(Some(Box::new(a)), Some(Box::new(b))).unwrap()
        );
    }
}
