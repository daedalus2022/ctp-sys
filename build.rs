fn main() {
    // ctp 所在目录名称
    let ctp_path = "ctp";
    // 版本
    let ctp_version = "v6.6.5_20210924";
    // 平台名称
    let platform = if cfg!(target_family = "windows") { "win" } else { "linux" };
    // 架构
    let arch = if cfg!(target_arch = "x86_64") {
        "x64"
    } else {
        panic!("can not build on this platform, linux_x64 and windows_x64.")
    };

    // so 所在目录
    let so_path = format!("{}/{}/{}_{}/", ctp_path, ctp_version,platform,arch);

    // 行情
    copy_lib_to(&so_path, &String::from("thostmduserapi_se.so"));
    // 交易
    copy_lib_to(&so_path, &String::from("thosttraderapi_se.so"));

    // 告诉 rustc 需要 link thostmduserapi_se thosttraderapi_se
    println!("cargo:rustc-link-lib=thostmduserapi_se");
    println!("cargo:rustc-link-lib=thosttraderapi_se");

    // 告诉 cargo 当 wrapper.h 变化时重新运行，只有结构使用wrapper.h, 包含类定义使用wrapper.cpp
    println!("cargo:rerun-if-changed=wrapper.cpp");

    // 配置 bindgen，并生成 Bindings 结构
    let bindings = bindgen::Builder::default()
        .header("wrapper.cpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // 生成 Rust 代码
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Failed to write bindings");
}

///
/// 分发动态连接库
///
fn copy_lib_to(so_path: &String, so_filename: &String){
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let current_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let so_symlink_string = format!("{}/lib{}", out_dir, so_filename);
    let so_symlink = std::path::Path::new(&so_symlink_string);

    println!("cargo:rustc-link-search=native={}", out_dir);
    let target_so = format!("{}/{}", out_dir, so_filename);
    let source_so = format!("{}/{}{}", current_dir, so_path, so_filename);

    // println!("debug source_so:{:?}, target_so:{:?}", source_so, target_so);
    std::fs::copy(&source_so, &target_so).expect("failed to copy so to outdir");
    println!("cargo:resource={}", target_so);

    // println!("debug so_symlink:{:?} is exists:{:?}", so_symlink, so_symlink.try_exists().unwrap());
    if so_symlink.exists() {
        std::fs::remove_file(so_symlink).expect("symlink exists, but failed to remove it");
    }
    std::os::unix::fs::symlink(&format!("{}/{}{}", current_dir, so_path, so_filename), so_symlink).expect("failed to create new symlink");
}
