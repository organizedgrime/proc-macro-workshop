use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    println!("ast: {:#?}", ast);

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
