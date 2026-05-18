fn main() {
    println!("cargo:rustc-link-lib=dylib=neuralnetworks");

    let target = std::env::var("TARGET").unwrap_or_default();
    if !target.contains("android") {
        return;
    }

    let ndk = match std::env::var("ANDROID_NDK_HOME") {
        Ok(v) => v,
        Err(_) => return,
    };

    let arch_dir = if target.starts_with("aarch64") {
        "aarch64-linux-android"
    } else if target.starts_with("armv7") || target.starts_with("arm") {
        "arm-linux-androideabi"
    } else if target.starts_with("i686") {
        "i686-linux-android"
    } else if target.starts_with("x86_64") {
        "x86_64-linux-android"
    } else {
        return;
    };

    // NDK prebuilt host directory
    let host = if cfg!(target_os = "windows") {
        "windows-x86_64"
    } else if cfg!(target_os = "macos") {
        "darwin-x86_64"
    } else {
        "linux-x86_64"
    };

    // Point to API 31 stubs — minimum required for BATCH_MATMUL
    let stub_path = format!(
        "{}/toolchains/llvm/prebuilt/{}/sysroot/usr/lib/{}/31",
        ndk, host, arch_dir
    );
    println!("cargo:rustc-link-search=native={}", stub_path);
}
