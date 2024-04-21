// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.



fn main() {  
    let answer = current_favorite_color();  
    println!("My current favorite color is {}", answer);  
}  
  
fn current_favorite_color() -> String {  
    // 使用 to_string 方法将 &str 转换为 String  
    "blue".to_string()  
    // 或者使用 String::from 函数来创建 String  
    // String::from("blue")  
}