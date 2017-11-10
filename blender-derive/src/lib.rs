#![recursion_limit="128"]
#![feature(box_patterns)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

/// For internal use, because the code for reading writing big / small endians is duplicated
enum Endianness {
    Big,
    Small,
}

#[proc_macro_derive(BlenderRead)]
pub fn blender_read(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_blender_read(&ast);
    println!("{}", gen);
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

    if let Body::Struct(VariantData::Struct(ref fields)) = ast.body {

        // implement big endian
        output_tokens.append(quote! { impl BlenderRead for #name });
        output_tokens.append("{");
        output_tokens.append("fn read_big_endian<R: ::std::io::Read>(mut buffer: R) -> Self");
        output_tokens.append("{");
        output_tokens.append(quote! { 
            let mut blender_data: #name = unsafe { ::std::mem::zeroed() };
        });

        // println!("fields: {:#?}", fields);

        // this section generates the "blender_data.name = buffer.read_u16::<BigEndian>().unwrap();"
        for field in fields.iter() {

            let ident = &field.ident;
            let ty = &field.ty;
            
            output_tokens.append(quote! {
                blender_data.#ident =
            });

            use syn::Ty::{Array, Path};
            
            match *ty {
                Path(_, ref path) => {
                    let field_type = &path.segments[0].ident; // "char" or "u8"
                    // output_tokens.append(quote! { buffer.read_#field_type::<BigEndian>().unwrap(); });
                    let type_spec = if field_type.as_ref() == "u8" || field_type.as_ref() == "i8" { "" } else { "::<BigEndian>" }; 
                    output_tokens.append(&format!("buffer.read_{0}{1}().unwrap();", field_type.as_ref(), type_spec));
                },
                Array(box ref p, ref lit) => {
                    use syn::ConstExpr::Lit;
                    use syn::Lit::Int;
                    
                    let number_of_items = match *lit {
                        Lit(Int(ref num, _)) => *num,
                        _ => panic!("unsupported array type"),
                    };
                    
                    match *p {
                        Path(_, ref path) => {
                            let field_type = &path.segments[0].ident;
                            output_tokens.append("[");

                            for i in 0..number_of_items {                                
                                let type_spec = if field_type.as_ref() == "u8" || field_type.as_ref() == "i8" { "" } else { "::<BigEndian>" }; 
                                output_tokens.append(&format!("buffer.read_{0}{1}().unwrap()", field_type.as_ref(), type_spec));
                                if i != (number_of_items - 1) { output_tokens.append(", "); }
                            }
                            
                            output_tokens.append("];");                            
                        },
                        _ => panic!("unsupported array type, please use the format [u8; 3] (example)"),
                    }
                },
                _ => { println!("unknown field type"); },
            }
        } // end of fields

        //println!("output tokens: {}", output_tokens.as_str());
        output_tokens.append(quote! {            
            blender_data
        });
        
        output_tokens.append("}");

        output_tokens.append(quote! {
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
        });
        output_tokens.append("}");
        
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
