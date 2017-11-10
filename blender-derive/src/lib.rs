#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(BlenderRead)]
pub fn blender_read(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_blender_read(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(BlenderWrite)]
pub fn blender_write(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_blender_write(&ast);
    gen.parse().unwrap()
}

fn impl_blender_read(ast: &syn::DeriveInput) -> quote::Tokens {

    let name = &ast.ident;
    quote! {
        impl BlenderRead for #name {
            fn read_big_endian<R: ::std::io::Read>(mut buffer: R) -> Self {
                // TODO: use byteorder!
                let mut blender_data: #name = unsafe { ::std::mem::zeroed() };
                let blender_data_size = ::std::mem::size_of::<#name>();

                unsafe {
                    let blender_data_slice = ::std::slice::from_raw_parts_mut(
                        &mut blender_data as *mut _ as *mut u8,
                        blender_data_size
                    );
                    buffer.read_exact(blender_data_slice).unwrap();
                }

                blender_data
            }

            fn read_little_endian<R: ::std::io::Read>(mut buffer: R) -> Self {
                // TODO: use byteorder!
                let mut blender_data: #name = unsafe { ::std::mem::zeroed() };
                let blender_data_size = ::std::mem::size_of::<#name>();

                unsafe {
                    let blender_data_slice = ::std::slice::from_raw_parts_mut(
                        &mut blender_data as *mut _ as *mut u8,
                        blender_data_size
                    );
                    buffer.read_exact(blender_data_slice).unwrap();
                }

                blender_data
            }
        }
    }
}

fn impl_blender_write(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl BlenderWrite for #name {

            fn write_big_endian<W: ::std::io::Write>(&self, target: W) -> Result<(), ::std::io::Error> {
                Ok(())
            }
            
            fn write_little_endian<W: ::std::io::Write>(&self, target: W) -> Result<(), ::std::io::Error> {
                Ok(())
            }
        }
    }
}
