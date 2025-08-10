# java-classfile

A simple Java Class file decoder for Rust.

## Features

- No dependencies.
- Zero copy.
- Comlies [Java 17 Specs.](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html)

## Usage
```rust
use java_classfile::decode;
use java_classfile::types::JavaClassFile;

let bytes: [u8] = [...];
let java_class_file: JavaClassFile = decode(&bytes);
```

## Supportes Features

- All Constant Pool entries.
- [Attributes](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7)
  - We have **NOT** supported decoding Attributes, but we are ready to implement in some days.
  1) Critical to correct interpretation
    - [ ] ConstantValue
    - [ ] Code
    - [ ] StackMapTable
    - [ ] BootstrapMethods
    - [ ] NestHost
    - [ ] NestMembers
    - [ ] PermittedSubclasses
  2) Critical to correct interpretation of the class files of Java SE Platform
    - [ ] Exceptions
    - [ ] InnerClasses
    - [ ] EnclosingMethod
    - [ ] Synthetic
    - [ ] Signature
    - [ ] Record
    - [ ] Sourcefile
    - [ ] LineNumberTable
    - [ ] LocalVariableTable
    - [ ] LocalVariableTypeTable
  3) Not critical to correct interpretation but contain metadata
    - [ ] SourceDebugExtension
    - [ ] Deprecated
    - [ ] RuntimeVisibleAnnotations
    - [ ] RuntimeInvisibleAnnotations
    - [ ] RuntimeVisibleParameterAnnotations
    - [ ] RuntimeInvisibleParameterAnnotations
    - [ ] RuntimeVisibleTypeAnnotations
    - [ ] RuntimeInvisibleTypeAnnotations
    - [ ] AnnotationDefault
    - [ ] MethodParameters
    - [ ] Module
    - [ ] ModulePackages
    - [ ] ModuleMainClass
