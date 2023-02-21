/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-20 20:19:09
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-20 20:29:57
 * @FilePath: \macro_workspace\hello_macro_derive\src\lib.rs
 * @Description: 
 */

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {

    // 将Rust代码转换为我们能够进行处理的语法树
    let ast = syn::parse(input).unwrap();

    // 构造对应的trait实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}