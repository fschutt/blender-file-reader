#![recursion_limit="128"]
#![feature(box_patterns)]

#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

/// For internal use, because the code for reading writing
/// big / small endians is duplicated
enum Endianness {
    Big,
    Little,
}

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
    let mut output_tokens = quote::Tokens::new();
    output_tokens.append(quote! { impl BlenderRead for #name });
    output_tokens.append("{");
    output_tokens.append(blender_impl_read_functions(ast, Endianness::Big));
    output_tokens.append(blender_impl_read_functions(ast, Endianness::Little));
    output_tokens.append("}");
    output_tokens
}

fn blender_impl_read_functions(ast: &syn::DeriveInput, endian: Endianness) -> quote::Tokens {
    use syn::{Body, VariantData};
    
    let mut output_tokens = quote::Tokens::new();
    let name = &ast.ident;
    // create variables based on endiannes
    let parse_str = match endian {
        Endianness::Big => "::<BigEndian>",
        Endianness::Little => "::<LittleEndian>",
    };

    let function_name = match endian {
        Endianness::Big => "big",
        Endianness::Little => "little",
    };
    
    if let Body::Struct(VariantData::Struct(ref fields)) = ast.body {    
        output_tokens.append(&format!("fn read_{}_endian<R: ::std::io::Read>(source: &mut R) -> Result<Self, BlenderError>", function_name));
        output_tokens.append("{");
        output_tokens.append("Ok(");
        output_tokens.append(quote! { 
            #name
        });
        output_tokens.append("{");
        // println!("fields: {:#?}", fields);

        // this section generates the "blender_data.name = buffer.read_u16::<BigEndian>().unwrap();"
        for field in fields.iter() {

            let ident = &field.ident;
            let ty = &field.ty;
            
            output_tokens.append(quote! {
                #ident : 
            });

            use syn::Ty::{Array, Path};
            
            match *ty {
                
                Path(_, ref path) => {
                    let field_type = &path.segments[0].ident; // "char" or "u8"
                    let type_spec = match field_type.as_ref() {
                        "u8" | "i8" => "",
                        _ => parse_str,
                    };
                    output_tokens.append(&format!("source.read_{0}{1}()?,", field_type.as_ref(), type_spec));
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
                                let type_spec = match field_type.as_ref() {
                                    "u8" | "i8" => "",
                                    _ => parse_str,
                                };
                                output_tokens.append(&format!("source.read_{0}{1}()?", field_type.as_ref(), type_spec));
                                if i != (number_of_items - 1) { output_tokens.append(", "); }
                            }
                            
                            output_tokens.append("],");                            
                        },
                        _ => panic!("unsupported array type, please use the format [u8; 3] (example)"),
                    }
                },
                _ => { println!("unknown field type"); },
            }
        } // end of fields

        output_tokens.append("})");
                
        output_tokens.append("}");
    }
    
    output_tokens
}


fn impl_blender_write(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut output_tokens = quote::Tokens::new();
    output_tokens.append(quote! { impl BlenderWrite for #name });
    output_tokens.append("{");
    output_tokens.append(blender_impl_write_functions(ast, Endianness::Big));
    output_tokens.append(blender_impl_write_functions(ast, Endianness::Little));
    output_tokens.append("}");
    output_tokens
}


fn blender_impl_write_functions(ast: &syn::DeriveInput, endian: Endianness) -> quote::Tokens {
    use syn::{Body, VariantData};
    
    let mut output_tokens = quote::Tokens::new();

    // create variables based on endiannes
    let parse_str = match endian {
        Endianness::Big => "::<BigEndian>",
        Endianness::Little => "::<LittleEndian>",
    };

    let function_name = match endian {
        Endianness::Big => "big",
        Endianness::Little => "little",
    };
    
    if let Body::Struct(VariantData::Struct(ref fields)) = ast.body {    
        output_tokens.append(&format!("fn write_{}_endian<W: ::std::io::Write>(&self, target: &mut W) -> Result<(), BlenderError>", function_name));
        output_tokens.append("{");

        // this section generates the "blender_data.name = target.write_u16::<BigEndian>().unwrap();"
        for field in fields.iter() {

            let field_name = field.ident.clone().unwrap();
            let ty = &field.ty;
            
            use syn::Ty::{Array, Path};
            
            match *ty {
                
                Path(_, ref path) => {
                    let field_type = &path.segments[0].ident; // "char" or "u8"
                    let type_spec = match field_type.as_ref() {
                        "u8" | "i8" => "",
                        _ => parse_str,
                    };
                    output_tokens.append(&format!("target.write_{0}{1}(self.{2})?;",
                                                  field_type.as_ref(), type_spec, field_name));
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
                            for i in 0..number_of_items {                                
                                let type_spec = match field_type.as_ref() {
                                    "u8" | "i8" => "",
                                    _ => parse_str,
                                };
                                output_tokens.append(&format!("target.write_{0}{1}(self.{2}[{3}])?;",
                                                              field_type.as_ref(), type_spec, field_name, i));
                            }
                        },
                        _ => panic!("unsupported array type, please use the format [u8; 3] (example)"),
                    }
                },
                _ => { println!("unknown field type"); },
            }
        } // end of fields
        output_tokens.append(quote!{ Ok(()) });
        output_tokens.append("}");
    }
    
    output_tokens
}
