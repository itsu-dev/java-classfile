use crate::utils::*;

/// Constant pool kinds as defined in the JVM specification.
/// 
/// ref. https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4-210
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstantKind {
    /// CONSTANT_Utf8
    /// since: class file format 45.3 (Java 1.0.2)
    Utf8 = 1,
    /// CONSTANT_Integer
    /// since: class file format 45.3 (Java 1.0.2)
    Integer = 3,
    /// CONSTANT_Float
    /// since: class file format 45.3 (Java 1.0.2)
    Float = 4,
    /// CONSTANT_Long
    /// since: class file format 45.3 (Java 1.0.2)
    Long = 5,
    /// CONSTANT_Double
    /// since: class file format 45.3 (Java 1.0.2)
    Double = 6,
    /// CONSTANT_Class
    /// since: class file format 45.3 (Java 1.0.2)
    Class = 7,
    /// CONSTANT_String
    /// since: class file format 45.3 (Java 1.0.2)
    String = 8,
    /// CONSTANT_FieldRef
    /// since: class file format 45.3 (Java 1.0.2)
    FieldRef = 9,
    /// CONSTANT_MethodRef
    /// since: class file format 45.3 (Java 1.0.2)
    MethodRef = 10,
    /// CONSTANT_InterfaceMethodRef
    /// since: class file format 45.3 (Java 1.0.2)
    InterfaceMethodRef = 11,
    /// CONSTANT_NameAndType
    /// since: class file format 45.3 (Java 1.0.2)
    NameAndType = 12,
    /// CONSTANT_MethodHandle
    /// since: class file format 51.0 (Java 7)
    MethodHandle = 15,
    /// CONSTANT_MethodType
    /// since: class file format 51.0 (Java 7)
    MethodType = 16,
    /// CONSTANT_Dynamic
    /// since: class file format 55.0 (Java 11)
    Dynamic = 17,
    /// CONSTANT_InvokeDynamic
    /// since: class file format 51.0 (Java 7)
    InvokeDynamic = 18,
    /// CONSTANT_Module
    /// since: class file format 53.0 (Java 9)
    Module = 19,
    /// CONSTANT_Package
    /// since: class file format 53.0 (Java 9)
    Package = 20,
}

impl From<u8> for ConstantKind {
    fn from(value: u8) -> Self {
        match value {
            1 => ConstantKind::Utf8,
            3 => ConstantKind::Integer,
            4 => ConstantKind::Float,
            5 => ConstantKind::Long,
            6 => ConstantKind::Double,
            7 => ConstantKind::Class,
            8 => ConstantKind::String,
            9 => ConstantKind::FieldRef,
            10 => ConstantKind::MethodRef,
            11 => ConstantKind::InterfaceMethodRef,
            12 => ConstantKind::NameAndType,
            15 => ConstantKind::MethodHandle,
            16 => ConstantKind::MethodType,
            17 => ConstantKind::Dynamic,
            18 => ConstantKind::InvokeDynamic,
            19 => ConstantKind::Module,
            20 => ConstantKind::Package,
            _ => panic!("Unknown ConstantKind value"),
        }
    }
}

/// Represents a constant pool entry in a Java class file.
/// 
/// ref. https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4-210
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantPoolInfo<'a> {
    Dummy(),
    /// CONSTANT_Class (tag: 7)
    /// since: class file format 45.3 (Java 1.0.2)
    Class(ConstantClassInfo),
    /// CONSTANT_FieldRef (tag: 9)
    /// since: class file format 45.3 (Java 1.0.2)
    FieldRef(ConstantFieldRefInfo),
    /// CONSTANT_MethodRef (tag: 10)
    /// since: class file format 45.3 (Java 1.0.2)
    MethodRef(ConstantMethodRefInfo),
    /// CONSTANT_InterfaceMethodRef (tag: 11)
    /// since: class file format 45.3 (Java 1.0.2)
    InterfaceMethodRef(ConstantInterfaceMethodRefInfo),
    /// CONSTANT_String (tag: 8)
    /// since: class file format 45.3 (Java 1.0.2)
    String(ConstantStringInfo),
    /// CONSTANT_Integer (tag: 3)
    /// since: class file format 45.3 (Java 1.0.2)
    Integer(ConstantIntegerInfo),
    /// CONSTANT_Float (tag: 4)
    /// since: class file format 45.3 (Java 1.0.2)
    Float(ConstantFloatInfo),
    /// CONSTANT_Long (tag: 5)
    /// since: class file format 45.3 (Java 1.0.2)
    Long(ConstantLongInfo),
    /// CONSTANT_Double (tag: 6)
    /// since: class file format 45.3 (Java 1.0.2)
    Double(ConstantDoubleInfo),
    /// CONSTANT_NameAndType (tag: 12)
    /// since: class file format 45.3 (Java 1.0.2)
    NameAndType(ConstantNameAndTypeInfo),
    /// CONSTANT_Utf8 (tag: 1)
    /// since: class file format 45.3 (Java 1.0.2)
    Utf8(ConstantUtf8Info<'a>),
    /// CONSTANT_MethodHandle (tag: 15)
    /// since: class file format 51.0 (Java 7)
    MethodHandle(ConstantMethodHandleInfo),
    /// CONSTANT_MethodType (tag: 16)
    /// since: class file format 51.0 (Java 7)
    MethodType(ConstantMethodTypeInfo),
    /// CONSTANT_Dynamic (tag: 17)
    /// since: class file format 55.0 (Java 11)
    Dynamic(ConstantDynamicInfo),
    /// CONSTANT_InvokeDynamic (tag: 18)
    /// since: class file format 51.0 (Java 7)
    InvokeDynamic(ConstantInvokeDynamicInfo),
    /// CONSTANT_Module (tag: 19)
    /// since: class file format 53.0 (Java 9)
    Module(ConstantModuleInfo),
    /// CONSTANT_Package (tag: 20)
    /// since: class file format 53.0 (Java 9)
    Package(ConstantPackageInfo),
}

/// CONSTANT_Class (tag: 7)
/// since: class file format 45.3 (Java 1.0.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantClassInfo {
    pub tag: ConstantKind,
    pub name_index: usize,
}

/// CONSTANT_FieldRef (tag: 9)
/// since: class file format 45.3 (Java 1.0.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantFieldRefInfo {
    pub tag: ConstantKind,
    pub class_index: usize,
    pub name_and_type_index: usize,
}

/// CONSTANT_MethodRef (tag: 10)
/// since: class file format 45.3 (Java 1.0.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantMethodRefInfo {
    pub tag: ConstantKind,
    pub class_index: usize,
    pub name_and_type_index: usize,
}

/// CONSTANT_InterfaceMethodRef (tag: 11)
/// since: class file format 45.3 (Java 1.0.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantInterfaceMethodRefInfo {
    pub tag: ConstantKind,
    pub class_index: usize,
    pub name_and_type_index: usize,
}

/// CONSTANT_String (tag: 8)
/// since: class file format 45.3 (Java 1.0.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantStringInfo {
    pub tag: ConstantKind,
    pub string_index: usize,
}

/// CONSTANT_Integer (tag: 3)
/// since: class file format 45.3 (Java 1.0.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantIntegerInfo {
    pub tag: ConstantKind,
    pub data: i32,
}

/// CONSTANT_Float (tag: 4)
/// since: class file format 45.3 (Java 1.0.2)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstantFloatInfo {
    pub tag: ConstantKind,
    pub data: f32,
}

/// CONSTANT_Long (tag: 5)
/// since: class file format 45.3 (Java 1.0.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantLongInfo {
    pub tag: ConstantKind,
    pub data: i64,
}

/// CONSTANT_Double (tag: 6)
/// since: class file format 45.3 (Java 1.0.2)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstantDoubleInfo {
    pub tag: ConstantKind,
    pub data: f64,
}

/// CONSTANT_NameAndType (tag: 12)
/// since: class file format 45.3 (Java 1.0.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantNameAndTypeInfo {
    pub tag: ConstantKind,
    pub name_index: usize,
    pub descriptor_index: usize,
}

/// CONSTANT_Utf8 (tag: 1)
/// since: class file format 45.3 (Java 1.0.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantUtf8Info<'a> {
    pub tag: ConstantKind,
    pub length: usize,
    pub data: &'a str,
}

/// CONSTANT_MethodHandle (tag: 15)
/// since: class file format 51.0 (Java 7)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantMethodHandleInfo {
    pub tag: ConstantKind,
    pub reference_kind: u8,
    pub reference_index: usize,
}

/// CONSTANT_MethodType (tag: 16)
/// since: class file format 51.0 (Java 7)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantMethodTypeInfo {
    pub tag: ConstantKind,
    pub descriptor_index: usize,
}

/// CONSTANT_Dynamic (tag: 17)
/// since: class file format 55.0 (Java 11)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantDynamicInfo {
    pub tag: ConstantKind,
    pub bootstrap_method_handle_attr_index: usize,
    pub name_and_type_index: usize,
}

/// CONSTANT_InvokeDynamic (tag: 18)
/// since: class file format 51.0 (Java 7)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantInvokeDynamicInfo {
    pub tag: ConstantKind,
    pub bootstrap_method_attr_index: usize,
    pub name_and_type_index: usize,
}

/// CONSTANT_Module (tag: 19)
/// since: class file format 53.0 (Java 9)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantModuleInfo {
    pub tag: ConstantKind,
    pub name_index: usize,
}

/// CONSTANT_Package (tag: 20)
/// since: class file format 53.0 (Java 9)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantPackageInfo {
    pub tag: ConstantKind,
    pub name_index: usize,
}

/// Decodes ConstantClassInfo
fn decode_class_info(buffer: &[u8]) -> (ConstantClassInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let name_index = read_u16(head) as usize;
    (
        ConstantClassInfo {
            tag: ConstantKind::Class,
            name_index,
        },
        rest,
    )
}

/// Decodes ConstantFieldRefInfo
fn decode_field_ref_info(buffer: &[u8]) -> (ConstantFieldRefInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let class_index = read_u16(head) as usize;
    let (head, rest) = rest.split_at(size_of::<u16>());
    let name_and_type_index = read_u16(head) as usize;
    (
        ConstantFieldRefInfo {
            tag: ConstantKind::FieldRef,
            class_index,
            name_and_type_index,
        },
        rest,
    )
}

/// Decodes ConstantMethodRefInfo
fn decode_method_ref_info(buffer: &[u8]) -> (ConstantMethodRefInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let class_index = read_u16(head) as usize;
    let (head, rest) = rest.split_at(size_of::<u16>());
    let name_and_type_index = read_u16(head) as usize;
    (
        ConstantMethodRefInfo {
            tag: ConstantKind::MethodRef,
            class_index,
            name_and_type_index,
        },
        rest
    )
}

/// Decodes ConstantInterfaceMethodRefInfo
fn decode_interface_method_ref_info(buffer: &[u8]) -> (ConstantInterfaceMethodRefInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let class_index = read_u16(head) as usize;
    let (head, rest) = rest.split_at(size_of::<u16>());
    let name_and_type_index = read_u16(head) as usize;
    (
        ConstantInterfaceMethodRefInfo {
            tag: ConstantKind::InterfaceMethodRef,
            class_index,
            name_and_type_index,
        },
        rest
    )
}

/// Decodes ConstantStringInfo
fn decode_string_info(buffer: &[u8]) -> (ConstantStringInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let string_index = read_u16(head) as usize;
    (
        ConstantStringInfo {
            tag: ConstantKind::String,
            string_index,
        },
        rest,
    )
}

/// Decodes ConstantIntegerInfo
fn decode_integer_info(buffer: &[u8]) -> (ConstantIntegerInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<i32>());
    let data = read_i32(head);
    (
        ConstantIntegerInfo {
            tag: ConstantKind::Integer,
            data,
        },
        rest,
    )
}

/// Decodes ConstantFloatInfo
fn decode_float_info(buffer: &[u8]) -> (ConstantFloatInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<f32>());
    let data = read_f32(head);
    (
        ConstantFloatInfo {
            tag: ConstantKind::Float,
            data,
        },
        rest,
    )
}
/// Decodes ConstantLongInfo
fn decode_long_info(buffer: &[u8]) -> (ConstantLongInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<i64>());
    let data = read_i64(head);
    (
        ConstantLongInfo {
            tag: ConstantKind::Long,
            data,
        },
        rest,
    )
}

/// Decodes ConstantDoubleInfo
fn decode_double_info(buffer: &[u8]) -> (ConstantDoubleInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<f64>());
    let data = read_f64(head);
    (
        ConstantDoubleInfo {
            tag: ConstantKind::Double,
            data,
        },
        rest,
    )
}

/// Decodes ConstantNameAndTypeInfo
fn decode_name_and_type_info(buffer: &[u8]) -> (ConstantNameAndTypeInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let name_index = read_u16(head) as usize;
    let (head, rest) = rest.split_at(size_of::<u16>());
    let descriptor_index = read_u16(head) as usize;
    (
        ConstantNameAndTypeInfo {
            tag: ConstantKind::NameAndType,
            name_index,
            descriptor_index,
        },
        rest,
    )
}

/// Decodes ConstantUtf8Info
fn decode_utf8_info<'a>(buffer: &'a [u8]) -> (ConstantUtf8Info<'a>, &'a [u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let length = read_u16(head) as usize;
    let (head, rest) = rest.split_at(length);
    let data = read_str(head);
    (
        ConstantUtf8Info {
            tag: ConstantKind::Utf8,
            length,
            data,
        },
        rest,
    )
}

/// Decodes ConstantMethodHandleInfo
fn decode_method_handle_info(buffer: &[u8]) -> (ConstantMethodHandleInfo, &[u8]) {
    let (head, rest) = buffer.split_at(1);
    let reference_kind = head[0];
    let (head, rest) = rest.split_at(size_of::<u16>());
    let reference_index = read_u16(head) as usize;
    (
        ConstantMethodHandleInfo {
            tag: ConstantKind::MethodHandle,
            reference_kind,
            reference_index,
        },
        rest,
    )
}

/// Decodes ConstantMethodTypeInfo
fn decode_method_type_info(buffer: &[u8]) -> (ConstantMethodTypeInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let descriptor_index = read_u16(head) as usize;
    (
        ConstantMethodTypeInfo {
            tag: ConstantKind::MethodType,
            descriptor_index,
        },
        rest,
    )
}

/// Decodes ConstantDynamicInfo
fn decode_dynamic_info(buffer: &[u8]) -> (ConstantDynamicInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let bootstrap_method_handle_attr_index = read_u16(head) as usize;
    let (head, rest) = rest.split_at(size_of::<u16>());
    let name_and_type_index = read_u16(head) as usize;
    (
        ConstantDynamicInfo {
            tag: ConstantKind::Dynamic,
            bootstrap_method_handle_attr_index,
            name_and_type_index,
        },
        rest,
    )
}

/// Decodes ConstantInvokeDynamicInfo
fn decode_invoke_dynamic_info(buffer: &[u8]) -> (ConstantInvokeDynamicInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let bootstrap_method_attr_index = read_u16(head) as usize;
    let (head, rest) = rest.split_at(size_of::<u16>());
    let name_and_type_index = read_u16(head) as usize;
    (
        ConstantInvokeDynamicInfo {
            tag: ConstantKind::InvokeDynamic,
            bootstrap_method_attr_index,
            name_and_type_index,
        },
        rest,
    )
}

/// Decodes ConstantModuleInfo
fn decode_module_info(buffer: &[u8]) -> (ConstantModuleInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let name_index = read_u16(head) as usize;
    (
        ConstantModuleInfo {
            tag: ConstantKind::Module,
            name_index,
        },
        rest,
    )
}

/// Decodes ConstantPackageInfo
fn decode_package_info(buffer: &[u8]) -> (ConstantPackageInfo, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let name_index = read_u16(head) as usize;
    (
        ConstantPackageInfo {
            tag: ConstantKind::Package,
            name_index,
        },
        rest,
    )
}

/// Decodes a constant pool.
pub(crate) fn decode_constant_pool(buffer: &[u8]) -> (Vec<ConstantPoolInfo>, &[u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let count = read_u16(head) as usize;

    let mut constants = Vec::with_capacity(count);
    constants.push(ConstantPoolInfo::Dummy());

    let mut i = 1;
    let mut buffer = rest;

    while i < count {
        let (head, rest) = buffer.split_at(1);
        let tag_byte = head[0];
        let tag = ConstantKind::from(tag_byte);

        match tag {
            ConstantKind::Class => {
                let (info, rest) = decode_class_info(rest);
                constants.push(ConstantPoolInfo::Class(info));
                buffer = rest;
            }

            ConstantKind::FieldRef => {
                let (info, rest) = decode_field_ref_info(rest);
                constants.push(ConstantPoolInfo::FieldRef(info));
                buffer = rest;
            }

            ConstantKind::MethodRef => {
                let (info, rest) = decode_method_ref_info(rest);
                constants.push(ConstantPoolInfo::MethodRef(info));
                buffer = rest;
            }

            ConstantKind::InterfaceMethodRef => {
                let (info, rest) = decode_interface_method_ref_info(rest);
                constants.push(ConstantPoolInfo::InterfaceMethodRef(info));
                buffer = rest;
            }

            ConstantKind::String => {
                let (info, rest) = decode_string_info(rest);
                constants.push(ConstantPoolInfo::String(info));
                buffer = rest;
            }

            ConstantKind::Integer => {
                let (info, rest) = decode_integer_info(rest);
                constants.push(ConstantPoolInfo::Integer(info));
                buffer = rest;
            }

            ConstantKind::Float => {
                let (info, rest) = decode_float_info(rest);
                constants.push(ConstantPoolInfo::Float(info));
                buffer = rest;
            }

            ConstantKind::Long => {
                let (info, rest) = decode_long_info(rest);
                constants.push(ConstantPoolInfo::Long(info));
                constants.push(ConstantPoolInfo::Dummy());
                buffer = rest;
                i += 1;
            }

            ConstantKind::Double => {
                let (info, rest) = decode_double_info(rest);
                constants.push(ConstantPoolInfo::Double(info));
                constants.push(ConstantPoolInfo::Dummy());
                buffer = rest;
                i += 1;
            }

            ConstantKind::NameAndType => {
                let (info, rest) = decode_name_and_type_info(rest);
                constants.push(ConstantPoolInfo::NameAndType(info));
                buffer = rest;
            }

            ConstantKind::Utf8 => {
                let (info, rest) = decode_utf8_info(rest);
                constants.push(ConstantPoolInfo::Utf8(info));
                buffer = rest;
            }

            ConstantKind::MethodHandle => {
                let (info, rest) = decode_method_handle_info(rest);
                constants.push(ConstantPoolInfo::MethodHandle(info));
                buffer = rest;
            }

            ConstantKind::MethodType => {
                let (info, rest) = decode_method_type_info(rest);
                constants.push(ConstantPoolInfo::MethodType(info));
                buffer = rest;
            }

            ConstantKind::Dynamic => {
                let (info, rest) = decode_dynamic_info(rest);
                constants.push(ConstantPoolInfo::Dynamic(info));
                buffer = rest;
            }

            ConstantKind::InvokeDynamic => {
                let (info, rest) = decode_invoke_dynamic_info(rest);
                constants.push(ConstantPoolInfo::InvokeDynamic(info));
                buffer = rest;
            }

            ConstantKind::Module => {
                let (info, rest) = decode_module_info(rest);
                constants.push(ConstantPoolInfo::Module(info));
                buffer = rest;
            }

            ConstantKind::Package => {
                let (info, rest) = decode_package_info(rest);
                constants.push(ConstantPoolInfo::Package(info));
                buffer = rest;
            }
        }

        i += 1;
    }

    (constants, buffer)
}

macro_rules! utf8_info_as_str {
    ($constant_pool:expr, $index:expr) => {
        match &$constant_pool[$index] {
            ConstantPoolInfo::Utf8(utf8_info) => utf8_info.data,
            _ => panic!("Not Utf8 ConstantPool Error"),
        }
    };
}

pub(crate) use utf8_info_as_str;