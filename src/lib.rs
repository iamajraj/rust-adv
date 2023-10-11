use proc_macro::TokenStream;

#[proc_macro]
pub fn html(_item: TokenStream) -> TokenStream {
    println!(
        "{:?}",
        _item.clone().to_string().replace(" ", "").replace("\"", "")
    );
    "()".parse().unwrap()
}

#[proc_macro_attribute]
pub fn main(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_str = input.clone().to_string();
    let fn_to_call = input_str.split(" ").nth(1).unwrap_or_else(|| "");
    println!("{:}", fn_to_call);
    // let x = format!(
    //     r#"
    //     fn main(){{
    //             {input}

    //             {fn_to_call};
    //     }}
    // "#,
    //     input = input,
    //     fn_to_call = fn_to_call
    // );

    // x.parse().unwrap()
    input
}
