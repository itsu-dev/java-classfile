use std::collections::HashMap;
use crate::{types::{decode_attributes, AttributeInfo, ConstantPoolInfo}, utils::read_u16};

pub const CLASS_FILE_MAGIC: u32 = 0xCAFEBABE;

/// The minor version of the Java class file.
pub const SUPPORTED_MINOR_VERSION: f32 = 45.0;

/// The major version of the Java class file.
pub const SUPPORTED_MAJOR_VERSION: f32 = 61.0;

/// Access flags for class.
/// 
/// ref. https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1-200-E.1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassAccessFlag {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
    Module = 0x8000,
}

/// Access flags for fields.
/// 
/// ref. https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.5-200-A.1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldAccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000,
}

/// Access flags for methods.
/// 
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.6-200-A.1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodAccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}

/// Trait for access flags.
pub trait AccessFlag {
    /// Tests if the flag has a specific access flag.
    fn test(&self, flag: u16) -> bool;
}

impl AccessFlag for ClassAccessFlag {
    /// Tests if the flag has a specific access flag.
    fn test(&self, flag: u16) -> bool {
        (*self as u16 & flag) != 0
    }
}

impl AccessFlag for FieldAccessFlag {
    /// Tests if the flag has a specific access flag.
    fn test(&self, flag: u16) -> bool {
        (*self as u16 & flag) != 0
    }
}

impl AccessFlag for MethodAccessFlag {
    /// Tests if the flag has a specific access flag.
    fn test(&self, flag: u16) -> bool {
        (*self as u16 & flag) != 0
    }
}

#[derive(Debug, Clone)]
pub struct Descriptor<'a> {
    pub is_primitive: bool,
    pub is_array: bool,
    pub type_name: &'a str,
}

#[derive(Debug)]
pub struct FieldInfo<'a> {
    pub access_flags: u16,
    pub name_index: usize,
    pub descriptor_index: usize,
    pub attributes: HashMap<u16, AttributeInfo<'a>>,
}

#[derive(Debug)]
pub struct MethodInfo<'a> {
    pub access_flags: u16,
    pub name_index: usize,
    pub descriptor_index: usize,
    pub attributes: HashMap<u16, AttributeInfo<'a>>,
}

/// Represents a Java class file.
/// 
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
#[derive(Debug)]
pub struct JavaClassFile<'a> {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<ConstantPoolInfo<'a>>,
    pub access_flags: u16,
    pub this_class: usize,
    pub super_class: usize,
    pub interfaces: Vec<usize>,
    pub fields: Vec<FieldInfo<'a>>,
    pub methods: Vec<MethodInfo<'a>>,
    pub attributes: HashMap<u16, AttributeInfo<'a>>,
}

impl<'a> JavaClassFile<'a> {
    /// Creates an empty JavaClassFile for primitive types.
    pub fn empty() -> JavaClassFile<'a> {
        Self {
            magic: CLASS_FILE_MAGIC,
            minor_version: 0,
            major_version: 0,
            constant_pool: Vec::new(),
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            attributes: HashMap::new(),
        }
    }
}

/// Decodes this_class or super_class
pub(crate) fn decode_this_or_super_class(buffer: &[u8]) -> (usize, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let class_index = read_u16(head) as usize;
    (class_index, rest)
}

/// Decodes interfaces
pub(crate) fn decode_interfaces(buffer: &[u8]) -> (Vec<usize>, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let interfaces_count = read_u16(head) as usize;
    let mut interfaces = Vec::with_capacity(interfaces_count);

    let mut buffer = rest;
    for _ in 0..interfaces_count {
        let (head, rest) = buffer.split_at(size_of::<u16>());
        let interface_index = read_u16(head) as usize;
        interfaces.push(interface_index);
        buffer = rest;
    }

    (interfaces, buffer)
}

/// Decodes fields
pub(crate) fn decode_fields<'a>(buffer: &'a [u8], constant_pool: &[ConstantPoolInfo]) -> (Vec<FieldInfo<'a>>, &'a [u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let fields_count = read_u16(head) as usize;
    let mut fields = Vec::with_capacity(fields_count);
    
    let mut buffer = rest;
    for _ in 0..fields_count {
        let (head, rest) = buffer.split_at(size_of::<u16>());
        let access_flags = read_u16(head);
        let (head, rest) = rest.split_at(size_of::<u16>());
        let name_index = read_u16(head) as usize;
        let (head, rest) = rest.split_at(size_of::<u16>());
        let descriptor_index = read_u16(head) as usize;
        let (attributes, rest) = decode_attributes(rest, constant_pool);

        fields.push(FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        });

        buffer = rest;
    }

    (fields, buffer)
}

/// Decodes methods
pub(crate) fn decode_methods<'a>(buffer: &'a [u8], constant_pool: &[ConstantPoolInfo]) -> (Vec<MethodInfo<'a>>, &'a [u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let methods_count = read_u16(head) as usize;
    let mut methods = Vec::with_capacity(methods_count);

    let mut buffer = rest;
    for _ in 0..methods_count {
        let (head, rest) = buffer.split_at(size_of::<u16>());
        let access_flags = read_u16(head);
        let (head, rest) = rest.split_at(size_of::<u16>());
        let name_index = read_u16(head) as usize;
        let (head, rest) = rest.split_at(size_of::<u16>());
        let descriptor_index = read_u16(head) as usize;
        let (attributes, rest) = decode_attributes(rest, constant_pool);

        methods.push(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        });

        buffer = rest;
    }

    (methods, buffer)
}
