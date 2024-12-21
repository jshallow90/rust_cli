fn main() {
    let mut args:Vec<String> = Vec::new(); 
    args.push("-il".to_string());
    args.push("-k".to_string());
    args.push("-j".to_string());
    args.push("t.*T".to_string());
    args.push("test_file.txt".to_string());
    
    let mut pos_args: Vec<char> = Vec::new();
    let custom_args: Vec<_> = args
        .iter()
        .filter(|arg| arg.starts_with("-"))
        .map(|arg| arg.replace("-", ""))
        .collect();

    for arg in &custom_args {
        for inner_arg in arg.chars() {
            pos_args.push(inner_arg)
        }
    }
    
    // let split_args = arg.chars();
    println!("args: {:?}, pos_args: {:?}", args, pos_args);
    println!("custom_args: {:?}", custom_args);
}
