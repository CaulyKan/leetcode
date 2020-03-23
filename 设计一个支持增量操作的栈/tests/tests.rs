extern crate leetcode;
use leetcode::CustomStack;

#[test]
fn test1() {
    let mut stack = CustomStack::new(3);
    stack.push(1); // 栈变为 [1]
    stack.push(2); // 栈变为 [1, 2]
    assert_eq!(2, stack.pop()); // 返回 2 --> 返回栈顶值 2，栈变为 [1]
    stack.push(2); // 栈变为 [1, 2]
    stack.push(3); // 栈变为 [1, 2, 3]
    stack.push(4); // 栈仍然是 [1, 2, 3]，不能添加其他元素使栈大小变为 4
    stack.increment(5, 100); // 栈变为 [101, 102, 103]
    stack.increment(2, 100); // 栈变为 [201, 202, 103]
    assert_eq!(103, stack.pop()); // 返回 103 --> 返回栈顶值 103，栈变为 [201, 202]
    assert_eq!(202, stack.pop()); // 返回 202 --> 返回栈顶值 202，栈变为 [201]
    assert_eq!(201, stack.pop()); // 返回 201 --> 返回栈顶值 201，栈变为 []
    assert_eq!(-1, stack.pop()); // 返回 -1 --> 栈为空，返回 -1
}
