use std::collections::HashMap;

use crate::{types::{utf8_info_as_str, ConstantPoolInfo}, utils::read_u16};

#[derive(Debug)]
pub enum AttributeInfo<'a> {
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttribute<'a>),
    StackMapTable(StackMapTableAttribute),
    Exceptions(ExceptionsAttribute),
    InnerClasses(InnerClassesAttribute),
    EnclosingMethod(EnclosingMethodAttribute),
    Synthetic(SyntheticAttribute),
    Signature(SignatureAttribute),
    SourceFile(SourceFileAttribute),
    // SourceDebugExtension(SourceDebugExtensionAttribute),
    LineNumberTable(LineNumberTableAttribute),
    LocalVariableTable(LocalVariableTableAttribute),
    LocalVariableTypeTable(LocalVariableTypeTableAttribute),
    // Deprecated(DeprecatedAttribute),
    // RuntimeVisibleAnnotations(RuntimeVisibleAnnotationsAttribute),
    // RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotationsAttribute),
    // RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotationsAttribute),
    // RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotationsAttribute),
    // RuntimeVisibleTypeAnnotationsAttribute(RuntimeVisibleTypeAnnotationsAttribute),
    // RuntimeInvisibleTypeAnnotationsAttribute(RuntimeInvisibleTypeAnnotationsAttribute),
    // AnnotationDefault(AnnotationDefaultAttribute),
    BootstrapMethods(BootstrapMethodsAttribute),
    // MethodParameters(MethodParametersAttribute),
    // Module,
    // ModulePackages,
    // ModuleMainClass,
    NestHost(NestHostAttribute),
    NestMembers(NestMembersAttribute),
    Record(RecordAttribute<'a>),
    PermittedSubtypes(PermittedSubtypesAttribute),
    Unknown,
}

#[derive(Debug)]
pub struct ConstantValueAttribute {
   pub constant_value_index: u16,
}

#[derive(Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug)]
pub struct CodeAttribute<'a> {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: usize,
    pub code: &'a [u8],
    pub exception_table_length: usize,
    pub exception_table: &'a [ExceptionTableEntry],
    pub attributes: HashMap<u16, AttributeInfo<'a>>,
}

#[derive(Debug)]
pub enum StackMapFrameType {
    SameFrame = 63,
    SameLocals1StackItemFrame = 127,
    SameLocals1StackItemFrameExtended = 247,
    ChopFrame = 250,
    SameFrameExtended = 251,
    AppendFrame = 254,
    FullFrame = 255,
}

#[derive(Debug)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object { cpool_index: u16 },
    Uninitialized { offset: u16 },
}

#[derive(Debug)]
pub struct SameFrame {
    pub frame_type: u8,  // 0-63
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrame {
    pub frame_type: u8,  // 64-127
    pub stack: VerificationTypeInfo,
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrameExtended {
    pub frame_type: u8,  // 247
    pub offset_delta: u16,
    pub stack: VerificationTypeInfo,
}

#[derive(Debug)]
pub struct ChopFrame {
    pub frame_type: u8,  // 248-250
    pub offset_delta: u16,
}

#[derive(Debug)]
pub struct SameFrameExtended {
    pub frame_type: u8,  // 251
    pub offset_delta: u16,
}

#[derive(Debug)]
pub struct AppendFrame {
    pub frame_type: u8,  // 252-254
    pub offset_delta: u16,
    pub locals: Vec<VerificationTypeInfo>,
}

#[derive(Debug)]
pub struct FullFrame {
    pub frame_type: u8,  // 255
    pub offset_delta: u16,
    pub number_of_locals: u16,
    pub locals: Vec<VerificationTypeInfo>,
    pub number_of_stack_items: u16,
    pub stack: Vec<VerificationTypeInfo>,
}

#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame(SameFrame),
    SameLocals1StackItemFrame(SameLocals1StackItemFrame),
    SameLocals1StackItemFrameExtended(SameLocals1StackItemFrameExtended),
    ChopFrame(ChopFrame),
    SameFrameExtended(SameFrameExtended),
    AppendFrame(AppendFrame),
    FullFrame(FullFrame),
}

#[derive(Debug)]
pub struct StackMapTableAttribute {
    pub number_of_entries: u16,
    pub entries: Vec<StackMapFrame>,
}

#[derive(Debug)]
pub struct ExceptionsAttribute {
    pub number_of_exceptions: u16,
    pub exception_index_table: Vec<u16>,
}

#[derive(Debug)]
pub struct InnerClassInfo {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

#[derive(Debug)]
pub struct InnerClassesAttribute {
    pub number_of_classes: u16,
    pub classes: Vec<InnerClassInfo>,
}

#[derive(Debug)]
pub struct EnclosingMethodAttribute {
    pub class_index: u16,
    pub method_index: u16,
}

#[derive(Debug)]
pub struct SyntheticAttribute;

#[derive(Debug)]
pub struct SignatureAttribute {
    pub signature_index: u16,
}

#[derive(Debug)]
pub struct SourceFileAttribute {
    pub sourcefile_index: u16,
}

// #[derive(Debug)]
// pub struct SourceDebugExtensionAttribute {
//     pub debug_extension: Vec<u8>,
// }

#[derive(Debug)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug)]
pub struct LineNumberTableAttribute {
    pub line_number_table_length: u16,
    pub line_number_table: Vec<LineNumberTableEntry>,
}

#[derive(Debug)]
pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: usize,
    pub descriptor_index: usize,
    pub index: usize,
}

#[derive(Debug)]
pub struct LocalVariableTableAttribute {
    pub local_variable_table_length: u16,
    pub local_variable_table: Vec<LocalVariableTableEntry>,
}

#[derive(Debug)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: usize,
    pub signature_index: usize,
    pub index: usize,
}

#[derive(Debug)]
pub struct LocalVariableTypeTableAttribute {
    pub local_variable_type_table_length: u16,
    pub local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

// #[derive(Debug)]
// pub struct DeprecatedAttribute;
//
// #[derive(Debug)]
// pub enum AnnotationElementValueEntryValue {
//     ConstValueIndex(u16),
//     EnumConstValue { type_name_index: u16, const_name_index: u16 },
//     ClassInfoIndex(u16),
//     AnnotationValue(AnnotationEntry),
//     ArrayValue { num_values: u16, values: Vec<AnnotationElementValueEntryValue> },
// }
//
// #[derive(Debug)]
// pub struct AnnotationElementValue {
//     pub tag: u8,
//     pub value: AnnotationElementValueEntryValue,
// }
//
// #[derive(Debug)]
// pub struct AnnotationElementValueEntry {
//     pub default_value: AnnotationElementValue,
// }
//
// #[derive(Debug)]
// pub struct AnnotationEntry {
//     pub type_index: u16,
//     pub num_element_value_pairs: u16,
// }
//
// #[derive(Debug)]
// pub struct RuntimeVisibleAnnotationsAttribute {
//     pub num_annotations: u16,
//     pub annotations: Vec<AnnotationEntry>,
// }
//
// #[derive(Debug)]
// pub struct RuntimeInvisibleAnnotationsAttribute {
//     pub num_annotations: u16,
//     pub annotations: Vec<AnnotationEntry>,
// }
//
// #[derive(Debug)]
// pub struct RuntimeVisibleParameterAnnotationsAttribute {
//     pub num_parameters: u8,
//     pub parameter_annotations: Vec<Vec<AnnotationEntry>>,
// }
//
// #[derive(Debug)]
// pub struct RuntimeInvisibleParameterAnnotationsAttribute {
//     pub num_parameters: u8,
//     pub parameter_annotations: Vec<Vec<AnnotationEntry>>,
// }
//
// #[derive(Debug)]
// pub struct RuntimeVisibleTypeAnnotationsAttribute {
//     pub num_annotations: u16,
//     pub annotations: Vec<AnnotationEntry>,
// }
//
// #[derive(Debug)]
// pub struct RuntimeInvisibleTypeAnnotationsAttribute {
//     pub num_annotations: u16,
//     pub annotations: Vec<AnnotationEntry>,
// }
//
//
// #[derive(Debug)]
// pub struct AnnotationDefaultAttribute {
//     pub default_value: AnnotationElementValue,
// }

#[derive(Debug)]
pub struct BootstrapMethodEntry {
    pub bootstrap_method_ref: usize,
    pub num_bootstrap_arguments: usize,
    pub bootstrap_arguments: Vec<usize>,
}

#[derive(Debug)]
pub struct BootstrapMethodsAttribute {
    pub num_bootstrap_methods: u16,
    pub bootstrap_methods: Vec<BootstrapMethodEntry>,
}

// #[derive(Debug)]
// pub struct MethodParametersEntry {
//     pub name_index: u16,
//     pub access_flags: u16,
// }
//
// #[derive(Debug)]
// pub struct MethodParametersAttribute {
//     pub parameters_count: u8,
//     pub parameters: Vec<MethodParametersEntry>,
// }

#[derive(Debug)]
pub struct NestHostAttribute {
    pub host_class_index: u16,
}

#[derive(Debug)]
pub struct NestMembersAttribute {
    pub number_of_classes: u16,
    pub classes: Vec<u16>,
}

#[derive(Debug)]
pub struct RecordComponentInfo<'a> {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: HashMap<u16, AttributeInfo<'a>>,
}

#[derive(Debug)]
pub struct RecordAttribute<'a> {
    pub components_count: u16,
    pub components: Vec<RecordComponentInfo<'a>>,
}

#[derive(Debug)]
pub struct PermittedSubtypesAttribute {
    pub number_of_classes: u16,
    pub classes: Vec<u16>,
}

/// Decodes attributes
/// TODO
pub fn decode_attributes<'a>(buffer: &'a [u8], constant_pool: &[ConstantPoolInfo]) -> (HashMap<u16, AttributeInfo<'a>>, &'a [u8]) {
    let (head, rest) = buffer.split_at(size_of::<u16>());
    let attributes_count = read_u16(head) as usize;
    let attributes: HashMap<u16, AttributeInfo<'a>> = HashMap::new();

    let mut buffer = rest;
    for _ in 0..attributes_count {
        let (head, rest) = buffer.split_at(size_of::<u16>());
        let attribute_name_index = read_u16(head);
        let attribute_name = utf8_info_as_str!(constant_pool, attribute_name_index as usize);
        let (head, rest) = rest.split_at(size_of::<u16>());
        let _attribute_length = read_u16(head) as usize;
        buffer = rest;

        match attribute_name {
            // "ConstantValue" => {
            //     let attribute_info = decode_constant_value_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "Code" => {
            //     let attribute_info = decode_code_attribute(buffer, constant_pool)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "StackMapTable" => {
            //     let attribute_info = decode_stack_map_table(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "BootstrapMethods" => {
            //     let attribute_info = decode_bootstrap_methods_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "NestHost" => {
            //     let attribute_info = decode_nest_host_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "NestMembers" => {
            //     let attribute_info = decode_nest_members_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "PermittedSubclasses" => {
            //     let attribute_info = decode_permitted_subclasses_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "Exceptions" => {
            //     let attribute_info = decode_exceptions_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "InnerClasses" => {
            //     let attribute_info = decode_inner_classes_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "EnclosingMethod" => {
            //     let attribute_info = decode_enclosing_method_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "Synthetic" => {
            //     let attribute_info = decode_synthetic_attribute()?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "Signature" => {
            //     let attribute_info = decode_signature_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "Record" => {
            //     let attribute_info = decode_record_attribute(buffer, constant_pool)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "SourceFile" => {
            //     let attribute_info = decode_source_file_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "LineNumberTable" => {
            //     let attribute_info = decode_line_number_table_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "LocalVariableTable" => {
            //     let attribute_info = decode_local_variable_table_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            // "LocalVariableTypeTable" => {
            //     let attribute_info = decode_local_variable_type_table_attribute(buffer)?;
            //     attributes.insert(attribute_name.to_string(), attribute_info);
            // },

            _ => {
                // let _attribute_info = AttributeInfo::Unknown;
                // let _ = buffer.split_to(attribute_length as usize);
                // attributes.insert(attribute_name.to_string(), attribute_info);
            }
        }
    }

    (attributes, buffer)
}