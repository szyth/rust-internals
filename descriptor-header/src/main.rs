// 1.10 — Memory layout: alignment, padding, repr(Rust) vs repr(C)
// Exercise: ABI-Correct, Size-Optimized Descriptor Header
// Spec: see §4 of "1.10 Memory layout - alignment, padding, repr(Rust) vs repr(C).md" in the vault (topics/).

#[repr(C)]
struct RawDescriptorHeader {
    report_id: u8,
    usage_page: u16,
    report_size: u32,
    report_count: u8,
}
// repr Rust
struct DescriptorStats {
    report_id: u8,
    usage_page: u16,
    report_size: u32,
    report_count: u8,
}
#[repr(C)]
struct RawDescriptorHeaderOptimized {
    report_id: u8,
    report_count: u8,
    usage_page: u16,
    report_size: u32,
}
#[repr(packed)]
struct RawDescriptorHeaderPacked {
    report_id: u8,
    report_count: u8,
    usage_page: u16,
    report_size: u32,
}
fn parse_packed(bytes: &[u8; 8]) -> (u8, u8, u16, u32) {
    let report_id = bytes[0];
    let report_count = bytes[1];
    let usage_page = u16::from_le_bytes([bytes[2], bytes[3]]);
    let report_size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

    (report_id, report_count, usage_page, report_size)
}

#[repr(transparent)]
struct ReportID(u8); // OR struct ReportID {report_id: u8}

fn main() {
    // for repr C:
    // align, per field: 1, 2, 4, 1 (size of field in bytes)
    // id  :  0 pad + 1 byte
    // page:  1 pad + 2 byte
    // size:  0 pad + 4 byte
    // count: 0 pad + 1 byte
    // align, per struct = max(field's alignment) = align of u32 = 4 bytes
    // struct: 9 bytes fields + 3 bytes pad  = 12 bytes (make it multiple of struct align ie 12 bytes)
    // memory: [I,.,P,P,S,S,S,S,C,.,.,.]

    // compile time check
    const _: () = assert!(std::mem::size_of::<RawDescriptorHeader>() == 12);

    // run time checks
    let size_of_repr_c = std::mem::size_of::<RawDescriptorHeader>();
    let size_of_repr_rust = std::mem::size_of::<DescriptorStats>();
    let size_of_repr_c_optimized = std::mem::size_of::<RawDescriptorHeaderOptimized>();
    let size_of_repr_c_packed = std::mem::size_of::<RawDescriptorHeaderPacked>();

    let struct_align = std::mem::align_of::<RawDescriptorHeader>();
    let id_offset = std::mem::offset_of!(RawDescriptorHeader, report_id);
    let page_offset = std::mem::offset_of!(RawDescriptorHeader, usage_page);
    let size_offset = std::mem::offset_of!(RawDescriptorHeader, report_size);
    let count_offset = std::mem::offset_of!(RawDescriptorHeader, report_count);

    println!("size_of_repr_c: {size_of_repr_c}"); // Size is 12 bytes so that Rust binary can be
    // compatible with C binary. the ABI rule.
    println!("size_of_repr_rust: {size_of_repr_rust}"); // 8 bytes, no manual optimization
    // required, because Rust uses it internally
    // so it can reoder struct fields as needed to lessen the struct size
    println!("size_of_repr_c_optimized: {size_of_repr_c_optimized}"); // 8 bytes, no field reorder by rust compiler, but
    // optimized
    println!("size_of_repr_c_packed: {size_of_repr_c_packed}");

    let header_packed = RawDescriptorHeaderPacked {
        report_id: 5,
        report_count: 10,
        usage_page: 50,
        report_size: 9,
    };

    println!(
        "01 Bytes packed fields access: allowed: {}",
        header_packed.report_count
    );
    // println!(
    //     "+2 Bytes packed fields access: denied: {}",
    //     header.usage_page
    // ); // E0793: reference to packed field is unaligned. packed structs are only aligned by one byte

    println!("struct_align: {struct_align}");
    println!("id_offset: {id_offset}");
    println!("page_offset: {page_offset}");
    println!("size_offset: {size_offset}");
    println!("count_offset: {count_offset}");

    // repr Transparent, only works for single field structs
    assert_eq!(size_of::<ReportID>(), size_of::<u8>(),);
    assert_eq!(align_of::<ReportID>(), align_of::<u8>(),);

    let packed_struct_bytes: [u8; 8] = unsafe { std::mem::transmute(header_packed) };

    let parsed = parse_packed(&packed_struct_bytes);
    assert_eq!(parsed, (5, 10, 50, 9));
}
