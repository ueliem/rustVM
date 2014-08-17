// enum Opcode {
//   Nop = 0x00,
//   Second = 0x01,
//   Third = 0x02
// }

struct BytecodeFile {
    magic_number: u32,

    minor_version: u16,
    major_version: u16,

    constant_pool_count: u16,

    // cp_info constant_pool[constant_pool_count - 1];

    // access_flags: u16,

    // this_class: u16,
    // super_class: u16,

    // interfaces_count: u16,
    //
    // u2 interfaces[interfaces_count];
    //
    // fields_count: u16,
    // field_info fields[fields_count];
    //
    // methods_count: u16,
    // method_info methods[methods_count];
    //
    // attributes_count: u16
    // attribute_info attributes[attributes_count];
}
