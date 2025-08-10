use crate::{types::*, utils::*};

mod attributes;
mod classfile;
mod constant_pool;

pub(crate) mod utils;

pub mod types {
    pub use crate::attributes::*;
    pub use crate::classfile::*;
    pub use crate::constant_pool::*;
}

/// Decode a Java class file from bytes.
pub fn decode(bytes: &[u8]) -> JavaClassFile {
    let (head, rest) = bytes.split_at(size_of::<u32>());
    let magic = read_u32(head);

    let (head, rest) = rest.split_at(size_of::<u16>());
    let minor_version = read_u16(head);

    let (head, rest) = rest.split_at(size_of::<u16>());
    let major_version = read_u16(head);
    
    let (constant_pool, rest) = decode_constant_pool(rest);
    
    let (head, rest) = rest.split_at(size_of::<u16>());
    let access_flags = read_u16(head);

    let (this_class, rest) = decode_this_or_super_class(rest);
    let (super_class, rest) = decode_this_or_super_class(rest);

    let (interfaces, rest) = decode_interfaces(rest);
    let (fields, rest) = decode_fields(rest, &constant_pool);
    let (methods, rest) = decode_methods(rest, &constant_pool);
    let (attributes, _) = decode_attributes(rest, &constant_pool);

    JavaClassFile {
        magic,
        minor_version,
        major_version,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields,
        methods,
        attributes,
    }
}

fn hoge() {
    let bytes = [0u8; 4];
    let java_class_file: JavaClassFile = decode(&bytes);
}