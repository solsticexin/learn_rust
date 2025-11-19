to
TO
 Rust 学习笔记 - 括号匹配算法

## 项目概述
实现一个有效的括号匹配验证算法，使用栈数据结构来检查字符串中的括号是否正确配对。

## 核心知识点

### 1. Rust 基础语法

#### 变量声明
```rust
let mut stack: Vec<char> = Vec::new();
```

#### 字符串迭代
```rust
for c in s.chars() {
    // &str 没有实现 Iterator，需要调用 .chars() 方法
}
```

### 2. Match 语句详解

#### 基本语法
```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_expression,
}
```

#### 多值匹配
```rust
match c {
    '(' | '[' | '{' => stack.push(c),  // 使用 | 匹配多个值
    ')' => { /* 处理右圆括号 */ },
    _ => {},  // 通配符，忽略其他字符
}
```

### 3. Vec 操作

#### 查看最后一个元素的方法

##### 方法1：`last()` - 推荐
```rust
if let Some(last_char) = stack.last() {
    // 安全访问，不移除元素
    println!("最后一个元素: {}", last_char);
}
```

##### 方法2：`pop()` - 移除并获取
```rust
if let Some(last_char) = stack.pop() {
    // 获取并移除最后一个元素
    println!("移除的元素: {}", last_char);
}
```

##### 方法3：索引访问
```rust
if !stack.is_empty() {
    let last_char = stack[stack.len() - 1];
}
```

### 4. Option 类型处理

#### 错误处理方式对比

##### `unwrap()` - 最暴力（不推荐）
```rust
stack.pop().unwrap()  // 如果为 None，程序会 panic
```

##### `expect()` - 更好的暴力
```rust
stack.pop().expect("栈为空但遇到右括号")  // 提供自定义错误信息
```

##### `is_some_and()` - 简洁优雅（推荐）
```rust
if stack.pop().is_some_and(|c| c != '(') { 
    return false; 
}
```

##### `if let` - 最安全
```rust
if let Some(c) = stack.pop() {
    if c != '(' {
        return false;
    }
} else {
    return false;
}
```

### 5. `is_some_and()` 函数详解

#### 语法
```rust
option.is_some_and(closure)
```

#### 功能
- 检查 Option 是否为 Some
- 如果是 Some(value)，执行闭包并返回结果
- 如果是 None，直接返回 false

#### 示例
```rust
// 检查栈顶是否不是期望的左括号
if stack.pop().is_some_and(|c| c != '(') {
    return false;  // 栈不为空且栈顶不是 '('
}
```

## 完整实现代码

```rust
fn is_valid_parentheses(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop().is_some_and(|c| c != '(') {
                    return false;
                }
            },
            ']' => {
                if stack.pop().is_some_and(|c| c != '[') {
                    return false;
                }
            },
            '}' => {
                if stack.pop().is_some_and(|c| c != '{') {
                    return false;
                }
            },
            _ => {}, // 忽略其他字符
        }
    }
    
    stack.is_empty()  // 如果栈为空，说明所有括号都匹配
}
```

## 测试用例

```rust
fn main() {
    let test_cases = vec![
        ("()", true),           // 简单匹配
        ("[]", true),
        ("{}", true),
        ("({[]})", true),       // 嵌套匹配
        ("{[])}", false),       // 不匹配
        ("([)]", false),        // 交叉不匹配
        ("((()", false),        // 不完整
        ("", true),             // 空字符串
        ("{[()]}", true),       // 复杂嵌套
        ("{[()])", false),      // 复杂不匹配
        (")", false),           // 只有右括号
        ("(", false),           // 只有左括号
    ];
    
    for (test_str, expected) in test_cases {
        let result = is_valid_parentheses(test_str);
        let status = if result == expected { "✓" } else { "✗" };
        println!("{} \"{}\": {} (期望: {})", status, test_str, result, expected);
    }
}
```

## 算法逻辑

1. **初始化**：创建空栈
2. **遍历字符串**：
   - 遇到左括号：压入栈
   - 遇到右括号：检查栈顶是否匹配
3. **匹配检查**：
   - 栈为空：返回 false
   - 栈顶不匹配：返回 false
   - 栈顶匹配：弹出栈顶元素
4. **最终检查**：栈为空则返回 true，否则返回 false

## 常见错误及解决方案

### 1. 字符串迭代错误
```rust
// 错误：&str 没有实现 Iterator
for c in s { }

// 正确：使用 .chars() 方法
for c in s.chars() { }
```

### 2. 类型不匹配
```rust
// 错误：类型声明错误
let mut stack: char = Vec::new();

// 正确：Vec<char>
let mut stack: Vec<char> = Vec::new();
```

### 3. 不安全的 Option 处理
```rust
// 危险：可能导致 panic
stack.pop().unwrap()

// 安全：使用 is_some_and()
stack.pop().is_some_and(|c| c != '(')
```

## 英文术语对照

| 中文 | 英文 |
|------|------|
| 括号 | Brackets/Parentheses |
| 圆括号 | Parentheses/Round Brackets |
| 方括号 | Square Brackets |
| 花括号 | Curly Braces/Braces |
| 栈 | Stack |
| 匹配 | Matching |
| 嵌套 | Nesting |
| 有效的 | Valid |
| 成功 | Success |
| 失败 | Failure |

## 学习要点总结

1. **Rust 语法**：掌握 match 语句、模式匹配、Option 类型处理
2. **数据结构**：理解栈的 LIFO 特性及应用场景
3. **算法思维**：学会使用栈解决括号匹配问题
4. **错误处理**：选择合适的 Option 处理方式
5. **代码质量**：编写安全、简洁、可读的代码

## 扩展练习

1. 支持更多类型的括号（如尖括号 <>）
2. 添加位置信息，指出具体哪个位置的括号不匹配
3. 优化性能，减少不必要的内存分配
4. 实现括号自动补全功能