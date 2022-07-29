use proc_macro2;
use proc_macro;
use quote::quote;
use syn::{self, Lit};






fn get_type_stringified(type_str: String, option: bool) -> proc_macro2::TokenStream {
    if option {
        match type_str.as_str() {
            "num" => quote! {Some(Type::Number)}.into(),
            "bool" => quote! {Some(Type::Boolean)}.into(),
            "string" => quote! {Some(Type::String)}.into(),
            "list" => quote! {Some(Type::List(Box::new(Type::Any)))}.into(),
            "any" => quote! {Some(Type::Any)}.into(),
            "" => quote! {None}.into(),
            s => quote! {Some(Type::Struct(#s))}.into()
        }
    }
    else {
        match type_str.as_str() {
            "num" => quote! {Type::Number}.into(),
            "bool" => quote! {Type::Boolean}.into(),
            "string" => quote! {Type::String}.into(),
            "list" => quote! {Type::List(Box::new(Type::Any))}.into(),
            "any" => quote! {Type::Any}.into(),
            s => quote! {Type::Struct(#s)}.into()
        }
    }
}







#[proc_macro_derive(SlothFunction, attributes(name, output, owner, module))]
pub fn sloth_function_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_sloth_function(&ast)
}



fn impl_sloth_function(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let struct_name = &ast.ident;
    let attributes = &ast.attrs;

    let mut name: String = "".to_string();
    let mut owner_type_str: String = "".to_string();
    let mut module_str: String = "".to_string();
    let mut output_type_str: String = "".to_string();
    

    for attr in attributes {
        let meta = attr.parse_meta().unwrap();
        match meta {
            syn::Meta::NameValue(mnv) => {
                if let Lit::Str(v) = mnv.lit {
                    match mnv.path.get_ident() {
                        Some(i) => {
                            match i.to_string().as_str() {
                                "name" => name = v.value(),
                                "owner" => owner_type_str = v.value(),
                                "module" => module_str = v.value(),
                                "output" => output_type_str = v.value(),
                                _ => ()
                            }
                        },
                        None => {}
                    }
                }
            },
            _ => {}
        }
    }

    if name.is_empty() {panic!("'name' must be defined when deriving SlothFunction")}
    if output_type_str.is_empty() {panic!("'output' must be defined when deriving SlothFunction")}

    let owner_type = get_type_stringified(owner_type_str, true);
    let output_type = get_type_stringified(output_type_str, false);


    let module = match module_str.as_str() {
        "" => quote! {None}.into(),
        v => quote! {Some(#v.to_string())}
    };

    let gen = quote! {
        impl SlothFunction for #struct_name {
            fn get_name(&self) -> String {#name.to_string()}
            fn get_owner_type(&self) -> Option<Type> {#owner_type}
            fn get_module(&self) -> Option<String> {#module}
            fn get_output_type(&self) -> Type {#output_type}

            fn get_signature(&self) -> FunctionSignature {
                FunctionSignature::new(self.get_module(), self.get_name(), self.get_owner_type(), None, Some(self.get_output_type()))
            }
        }
    };
    
    gen.into()
}