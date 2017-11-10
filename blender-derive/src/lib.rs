#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt, LittleEndian, WriteBytesExt};

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
    use syn::{Body, VariantData};
    let mut output_tokens = quote::Tokens::new();
    let name = &ast.ident;
    // buffer.read_u16::<BigEndian>().unwrap()
    if let Body::Struct(VariantData::Struct(ref fields)) = ast.body {

        // implement big endian
        output_tokens.append(quote! { impl #name });
        output_tokens.append("{");
        output_tokens.append("fn read_big_endian<R: ::std::io::Read>(mut buffer: R) -> Self");
        output_tokens.append("{");
        output_tokens.append(quote! { 
            let mut blender_data: #name = unsafe { ::std::mem::zeroed() };
        });
/*
        for field in fields.iter() {
            let ident = &field.ident;
            let ty = &field.ty;
            output_tokens.append(quote! {
                blender_data.#field
            });
            output_tokens.append(" = ");
            output_tokens.append(quote! { buffer.read_#ty::<BigEndian>().unwrap(); });
            /*
            match ty {
                Array(a) => {
                    //             output_tokens.append("");
                },
                
            }*/
        }
*/      
        output_tokens.append(quote! { 
            let blender_data_size = ::std::mem::size_of::<#name>();
            
            unsafe {
                let blender_data_slice = ::std::slice::from_raw_parts_mut(
                    &mut blender_data as *mut _ as *mut u8,
                    blender_data_size
                );
                buffer.read_exact(blender_data_slice).unwrap();
            }
            
            blender_data
        });
        
        output_tokens.append("}");
        output_tokens.append("}");

        output_tokens.append(quote! {
            impl BlenderRead for #name { 
                fn read_big_endian<R: ::std::io::Read>(mut buffer: R) -> Self {
                    #name::read_big_endian(buffer)
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
        });
        
    } else {
        panic!("#[derive(BlenderRead)] is only defined for structs");
    }

    output_tokens
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
