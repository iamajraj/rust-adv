use proc_macro::TokenStream;

#[proc_macro]
pub fn html(_item: TokenStream) -> TokenStream {
    println!(
        "{:?}",
        _item.clone().to_string().replace(" ", "").replace("\"", "")
    );
    // "()".parse().unwrap()
    let str = format!("{:?}", _item.clone().to_string());
    str.parse().unwrap()
}

#[proc_macro_attribute]
pub fn main(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_str = input.clone().to_string();

    // x.parse().unwrap()
    let fn_name = input_str.split(" ").nth(1).unwrap();

    let l_code = &input_str[11..input_str.len() - 1];
    let mut code = String::new();
    code.push_str(format!("fn {:}", fn_name).as_str());

    code.push_str(
        "let m_thread = std::thread::spawn(|| {
        for i in 1..10 {
            std::thread::sleep(std::time::Duration::from_millis(400));
            println!(\"{:}\", i);
        }
    });",
    );
    code.push_str("println!(\"Customized\");");

    code.push_str(l_code);
    code.push_str(";m_thread.join().unwrap();");
    code.push_str("}");

    code.parse().unwrap()
    // input
}
