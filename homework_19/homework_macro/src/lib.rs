extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr, Token, punctuated::Punctuated, parse::{Parse, ParseStream}};


// Определение структуры для парсинга входных данных
struct MacroInput {
    function_names: Punctuated<LitStr, Token![,]>,
}

// Реализация парсинга для MacroInput
impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let function_names = Punctuated::parse_terminated(input)?;
        Ok(MacroInput { function_names })
    }
}

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Парсинг входного потока в MacroInput
    let input = parse_macro_input!(input as MacroInput);

    // Фильтрация имён функций с чётным количеством символов
    let mut function_names = Vec::new();
    for lit_str in input.function_names {
        let func_name = lit_str.value();
        if func_name.len() % 2 == 0 {
            function_names.push(Ident::new(&func_name, lit_str.span()));
        }
    }

    // Создание токенов для вызова функций и формирования кортежа
    let calls = function_names.iter().map(|ident| {
        quote! {
            #ident()
        }
    });

    let output = quote! {
        ( #(#calls),* )
    };

    TokenStream::from(output)
}