error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }
    errors {
        InvalidBlendFile {
            description("Not a blender file")
                display("Invalid .blend file")
        }
    }
}
