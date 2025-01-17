fn main() {
    let root_dir = std::path::Path::new(".");
    let src_dir_block = root_dir.join("tree-sitter-markdown").join("src");
    let src_dir_inline = root_dir.join("tree-sitter-markdown-inline").join("src");

    let mut c_config = cc::Build::new();
    c_config.include(&src_dir_block);
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");
    let parser_path = src_dir_block.join("parser.c");
    c_config.file(&parser_path);
    c_config.compile("parser_block");
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    let mut c_config = cc::Build::new();
    c_config.include(&src_dir_inline);
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");
    let parser_path = src_dir_inline.join("parser.c");
    c_config.file(&parser_path);
    c_config.compile("parser_inline");
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    let mut cpp_config = cc::Build::new();
    cpp_config.cpp(true);
    cpp_config.include(&src_dir_block);
    cpp_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable");
    let scanner_path = src_dir_block.join("scanner.cc");
    cpp_config.file(&scanner_path);
    cpp_config.compile("scanner_block");
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());

    let mut cpp_config = cc::Build::new();
    cpp_config.cpp(true);
    cpp_config.include(&src_dir_inline);
    cpp_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable");
    let scanner_path = src_dir_inline.join("scanner.cc");
    cpp_config.file(&scanner_path);
    cpp_config.compile("scanner_inline");
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
}
