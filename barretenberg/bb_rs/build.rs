use cmake::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Fix duplicate type definitions in the generated bindings file
/// It's known bug with bindgen that generates duplicate type definitions
/// if they are defined in multiple templates.
/// It's easier to just post-process the bindings file to remove the duplicate type definitions,
/// rather than trying to patch for it in the C++ code.
fn fix_duplicate_bindings(bindings_file: &PathBuf) {
    println!("cargo:warning=Fixing duplicate type definitions in bindings...");
    
    let scripts_dir = PathBuf::from("scripts");
    let python_script = scripts_dir.join("fix_bindings.py");
    let shell_script = scripts_dir.join("fix_bindings.sh");
    
    // Try Python script first
    if python_script.exists() {
        let output = Command::new("python3")
            .arg(&python_script)
            .arg(bindings_file)
            .output();
            
        match output {
            Ok(result) => {
                if result.status.success() {
                    println!("cargo:warning=Successfully fixed bindings with Python script");
                    return;
                } else {
                    println!("cargo:warning=Python script failed, trying shell script...");
                }
            }
            Err(_) => {
                println!("cargo:warning=Python not available, trying shell script...");
            }
        }
    }
    
    // Fallback to shell script
    if shell_script.exists() {
        let output = Command::new("bash")
            .arg(&shell_script)
            .arg(bindings_file)
            .output();
            
        match output {
            Ok(result) => {
                if result.status.success() {
                    println!("cargo:warning=Successfully fixed bindings with shell script");
                } else {
                    println!("cargo:warning=Shell script failed");
                    eprintln!("Shell script stderr: {}", String::from_utf8_lossy(&result.stderr));
                }
            }
            Err(e) => {
                println!("cargo:warning=Failed to run shell script: {}", e);
            }
        }
    } else {
        println!("cargo:warning=No fix scripts found, skipping duplicate removal");
    }
}

fn main() {
    // Notify Cargo to rerun this build script if `build.rs` changes.
    println!("cargo:rerun-if-changed=build.rs");

    // cfg!(target_os = "<os>") does not work so we get the value
    // of the target_os environment variable to determine the target OS.
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Build the C++ code using CMake and get the build directory path.
    let dst;
    // iOS
    if target_os == "ios" {
        // Determine the correct platform based on the target architecture
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        let target = env::var("TARGET").unwrap_or_default();
        
        let platform = if target.contains("sim") {
            if target_arch == "aarch64" {
                "SIMULATORARM64"
            } else if target_arch == "x86_64" {
                "SIMULATOR64"
            } else {
                "SIMULATOR"
            }
        } else {
            "OS64"
        };
        
        dst = Config::new("../cpp")
            .generator("Ninja")
            .configure_arg("-DCMAKE_BUILD_TYPE=Release")
            .configure_arg(&format!("-DPLATFORM={}", platform))
            .configure_arg("-DDEPLOYMENT_TARGET=15.0")
            .configure_arg("--toolchain=../bb_rs/ios.toolchain.cmake")
            .configure_arg("-DTRACY_ENABLE=OFF")
            .build_target("bb")
            .build();
    }
    // Android
    else if target_os == "android" {
        let android_home = option_env!("ANDROID_HOME").expect("ANDROID_HOME not set");
        let ndk_version = option_env!("NDK_VERSION").expect("NDK_VERSION not set");

        dst = Config::new("../cpp")
        .generator("Ninja")
        .configure_arg("-DCMAKE_BUILD_TYPE=Release")
        .configure_arg("-DANDROID_ABI=arm64-v8a")
        .configure_arg("-DANDROID_PLATFORM=android-33")
        .configure_arg(&format!("--toolchain={}/ndk/{}/build/cmake/android.toolchain.cmake", android_home, ndk_version))
        .configure_arg("-DTRACY_ENABLE=OFF")
        .build_target("bb")
        .build();
    }
    // MacOS and other platforms
    else {
        dst = Config::new("../cpp")
        .generator("Ninja")
        .configure_arg("-DCMAKE_BUILD_TYPE=Release")            
        .configure_arg("-DTRACY_ENABLE=OFF")
        .build_target("bb")
        .build();
    }

    // Add the library search path for Rust to find during linking.
    println!("cargo:rustc-link-search={}/build/lib", dst.display());

    // Add the library search path for libdeflate
    println!("cargo:rustc-link-search={}/build/_deps/libdeflate-build", dst.display());

    // Link the `barretenberg` static library.
    println!("cargo:rustc-link-lib=static=barretenberg");

    // Link the `libdeflate` static library.
    println!("cargo:rustc-link-lib=static=deflate");

    // Link the C++ standard library.
    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }

    // Copy the headers to the build directory.
    // Fix an issue where the headers are not included in the build.
    Command::new("sh").args(&["copy-headers.sh", &format!("{}/build/include", dst.display())]).output().unwrap();

    let mut builder = bindgen::Builder::default();

    if target_os == "android" {
        let android_home = option_env!("ANDROID_HOME").expect("ANDROID_HOME not set");
        let ndk_version = option_env!("NDK_VERSION").expect("NDK_VERSION not set");
        let host_tag = option_env!("HOST_TAG").expect("HOST_TAG not set");

        builder = builder
        // Add the include path for headers.
        .clang_args([
            "-std=c++20",
            "-xc++",
            &format!("-I{}/build/include", dst.display()),
            // Dependencies' include paths needs to be added manually.
            &format!("-I{}/build/_deps/msgpack-c/src/msgpack-c/include", dst.display()),
            //&format!("-I{}/build/_deps/libdeflate-src", dst.display()),
            &format!("-I{}/ndk/{}/toolchains/llvm/prebuilt/{}/sysroot/usr/include/c++/v1", android_home, ndk_version, host_tag),
            &format!("-I{}/ndk/{}/toolchains/llvm/prebuilt/{}/sysroot/usr/include", android_home, ndk_version, host_tag),
            &format!("-I{}/ndk/{}/toolchains/llvm/prebuilt/{}/sysroot/usr/include/aarch64-linux-android", android_home, ndk_version, host_tag)
        ]);
    } else if target_os == "ios" {
        // Determine the correct platform SDK based on the target
        let target = env::var("TARGET").unwrap_or_default();
        let sdk_platform = if target.contains("sim") {
            "iPhoneSimulator"
        } else {
            "iPhoneOS"
        };
        
        builder = builder
        // Add the include path for headers.
        .clang_args([
            "-std=c++20",
            "-xc++",
            &format!("-I{}/build/include", dst.display()),
            // Dependencies' include paths needs to be added manually.
            &format!("-I{}/build/_deps/msgpack-c/src/msgpack-c/include", dst.display()),
            //&format!("-I{}/build/_deps/libdeflate-src", dst.display()),
            &format!("-I/Applications/Xcode.app/Contents/Developer/Platforms/{}.platform/Developer/SDKs/{}.sdk/usr/include/c++/v1", sdk_platform, sdk_platform),
            &format!("-I/Applications/Xcode.app/Contents/Developer/Platforms/{}.platform/Developer/SDKs/{}.sdk/usr/include", sdk_platform, sdk_platform)
        ]);
    } else if target_os == "macos" {
        builder = builder
            // Add the include path for headers.
            .clang_args([
                "-std=c++20",
                "-xc++",
                &format!("-I{}/build/include", dst.display()),
                // Dependencies' include paths needs to be added manually.
                &format!("-I{}/build/_deps/msgpack-c/src/msgpack-c/include", dst.display()),
                //&format!("-I{}/build/_deps/libdeflate-src", dst.display()),
                "-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/c++/v1",
                "-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include",
            ]);
    } else {
        builder = builder
        // Add the include path for headers.
        .clang_args([
            "-std=c++20",
            "-xc++",
            &format!("-I{}/build/include", dst.display()),
            // Dependencies' include paths needs to be added manually.
            &format!("-I{}/build/_deps/msgpack-c/src/msgpack-c/include", dst.display()),
            //&format!("-I{}/build/_deps/libdeflate-src", dst.display()),
        ]);
    }

    let bindings = builder
        // The input header we would like to generate bindings for.
        .header_contents(
            "wrapper.hpp",
            r#"
                #include <barretenberg/crypto/pedersen_commitment/c_bind.hpp>
                #include <barretenberg/crypto/pedersen_hash/c_bind.hpp>
                #include <barretenberg/crypto/poseidon2/c_bind.hpp>
                #include <barretenberg/crypto/blake2s/c_bind.hpp>
                #include <barretenberg/crypto/schnorr/c_bind.hpp>
                #include <barretenberg/srs/c_bind.hpp>
                #include <barretenberg/common/c_bind.hpp>
                #include <barretenberg/dsl/acir_proofs/c_bind.hpp>
            "#,
        )
        .allowlist_function("pedersen_commit")
        .allowlist_function("pedersen_hash")
        .allowlist_function("pedersen_hashes")
        .allowlist_function("pedersen_hash_buffer")
        .allowlist_function("poseidon2_hash")
        .allowlist_function("poseidon2_hashes")
        .allowlist_function("blake2s")
        .allowlist_function("blake2s_to_field_")
        .allowlist_function("schnorr_construct_signature")
        .allowlist_function("schnorr_verify_signature")
        .allowlist_function("schnorr_multisig_create_multisig_public_key")
        .allowlist_function("schnorr_multisig_validate_and_combine_signer_pubkeys")
        .allowlist_function("schnorr_multisig_construct_signature_round_1")
        .allowlist_function("schnorr_multisig_construct_signature_round_2")
        .allowlist_function("schnorr_multisig_combine_signatures")
        .allowlist_function("aes_encrypt_buffer_cbc")
        .allowlist_function("aes_decrypt_buffer_cbc")
        .allowlist_function("srs_init_srs")
        .allowlist_function("srs_init_grumpkin_srs")
        .allowlist_function("test_threads")
        .allowlist_function("common_init_slab_allocator")
        .allowlist_function("acir_get_circuit_sizes")
        .allowlist_function("acir_serialize_proof_into_fields")
        .allowlist_function("acir_serialize_verification_key_into_fields")
        .allowlist_function("acir_prove_ultra_zk_honk")
        .allowlist_function("acir_prove_ultra_keccak_honk")
        .allowlist_function("acir_prove_ultra_keccak_zk_honk")
        .allowlist_function("acir_prove_aztec_client")
        // TODO: enable the Starknet flavors once we enable the appropriate flag
        // for the build process.
        //.allowlist_function("acir_prove_ultra_starknet_honk")
        //.allowlist_function("acir_prove_ultra_starknet_zk_honk")
        .allowlist_function("acir_verify_ultra_zk_honk")
        .allowlist_function("acir_verify_ultra_keccak_honk")
        .allowlist_function("acir_verify_ultra_keccak_zk_honk")
        .allowlist_function("acir_verify_aztec_client")
        //.allowlist_function("acir_verify_ultra_starknet_honk")
        //.allowlist_function("acir_verify_ultra_starknet_zk_honk")
        .allowlist_function("acir_write_vk_ultra_honk")
        .allowlist_function("acir_write_vk_ultra_keccak_honk")
        .allowlist_function("acir_write_vk_ultra_keccak_zk_honk")
        //.allowlist_function("acir_write_vk_ultra_starknet_honk")
        //.allowlist_function("acir_write_vk_ultra_starknet_zk_honk")
        .allowlist_function("acir_prove_and_verify_ultra_honk")
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_file = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_file)
        .expect("Couldn't write bindings!");

    // Fix duplicate type definitions in the generated bindings
    fix_duplicate_bindings(&bindings_file);
}
