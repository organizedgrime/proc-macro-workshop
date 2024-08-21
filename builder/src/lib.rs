use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Ident, Type};

#[derive(Debug)]
struct Field {
    pub f_ident: Ident,
    pub f_type: Type,
    pub f_optional: bool,
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let option_ident = Ident::new("Option", Span::call_site().into());

    if let Data::Struct(data_struct) = ast.data.clone() {
        if let Fields::Named(namedfields) = data_struct.fields {
            //namedfields.named.into_iter().map(|v} v.)
            let mut fields: Vec<Field> = namedfields
                .named
                .into_iter()
                .map(|field| Field {
                    f_ident: field.ident.unwrap(),
                    f_type: field.ty,
                    f_optional: false,
                })
                .collect();

            for field in fields.iter_mut() {
                if let Type::Path(type_path) = &field.f_type {
                    let path_segment = &type_path.path.segments[0];
                    if path_segment.ident == option_ident {
                        if let syn::PathArguments::AngleBracketed(args) = &path_segment.arguments {
                            if let GenericArgument::Type(f_type) = &args.args[0] {
                                field.f_type = f_type.clone();
                                field.f_optional = true;
                            }
                        }
                    }
                }
            }
            println!("fields: {:#?}", fields);

            println!("WARNING: this AST isn't applicable!");
            return quote! {}.into();
        }
    }

    let ident = ast.ident.clone();
    let builder_ident = format_ident!("{}{}", ident, "Builder");

    quote! {
        pub struct #builder_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

        impl #builder_ident {
            fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }

            fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }

            fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }

            fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }

            fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                match (&self.executable, &self.args, &self.env, &self.current_dir) {
                    (Some(executable), Some(args), Some(env), Some(current_dir)) => Ok(#ident {
                        executable: executable.clone(),
                        args: args.clone(),
                        env: env.clone(),
                        current_dir: current_dir.clone()
                    }),
                    _ => Err("Fields remain uninitialized, cant build yet.".into())
                }
            }
        }
    }
    .into()
}
