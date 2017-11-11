error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Nul(::std::ffi::NulError);
    }
    errors {
        InvalidBlendFile {
            description("Not a blender file"),
            display("Invalid .blend file"),
        }
        InvalidPointerSize(ptr: char) {
            description("Invalid pointer size"),
                display("Pointer size is not '-' (64bit) or '_' (32bit), got '{}'", ptr),
        }
        InvalidEndianness(endian: char) {
            description("Invalid endianness"),
                display("Must be 'v' (little endian) or 'V' (big endian), got '{}'", endian),
        }
    }
}
