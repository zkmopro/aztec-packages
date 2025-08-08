use cmake::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Notify Cargo to rerun this build script if `build.rs` changes.
    println!("cargo:rerun-if-changed=build.rs");

    // cfg!(target_os = "<os>") does not work so we get the value
    // of the target_os environment variable to determine the target OS.
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target = env::var("TARGET").unwrap();

    // Build the C++ code using CMake and get the build directory path.
    let dst;
    // iOS
    if target_os == "ios" {
        let (platform, arch, cmake_arch, deployment_target, sdk) = if target == "x86_64-apple-ios" {
            ("SIMULATOR64", "x86_64", "x86_64", "13.0", "iphonesimulator")
        } else if target == "aarch64-apple-ios-sim" {
            (
                "SIMULATORARM64",
                "arm64",
                "arm64",
                "14.0",
                "iphonesimulator",
            )
        } else {
            ("OS64", "arm64", "arm64", "15.0", "iphoneos")
        };

        let sdk_path = String::from_utf8(
            Command::new("xcrun")
                .args(&["--sdk", sdk, "--show-sdk-path"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string();
        dst = Config::new("../cpp")
            .generator("Ninja")
            .configure_arg("-DCMAKE_BUILD_TYPE=Release")
            .configure_arg(&format!("-DPLATFORM={}", platform))
            .configure_arg(&format!("-DDEPLOYMENT_TARGET={}", deployment_target))
            .configure_arg(&format!("--toolchain=../cpp/ios.toolchain.cmake"))
            .define("CMAKE_SYSTEM_NAME", "iOS")
            .define("CMAKE_OSX_SYSROOT", &sdk_path)
            .define("CMAKE_OSX_ARCHITECTURES", cmake_arch)
            .define("CMAKE_SYSTEM_PROCESSOR", arch)
            .build_target("bb")
            .build();
    }
    // Android
    else if target_os == "android" {
        let android_home = option_env!("ANDROID_HOME").expect("ANDROID_HOME not set");
        let ndk_version = option_env!("NDK_VERSION").expect("NDK_VERSION not set");

        // Auto-detect host tag
        let host_tag = env::var("HOST_TAG").unwrap_or_else(|_| {
            let os = env::consts::OS;
            match os {
                "macos" => "darwin-x86_64".to_string(),
                "linux" => "linux-x86_64".to_string(),
                "windows" => "windows-x86_64".to_string(),
                _ => panic!("Unsupported host OS: {}", os),
            }
        });

        // Map Rust target to Android ABI
        let android_abi = match target.as_str() {
            "x86_64-linux-android" => "x86_64",
            "aarch64-linux-android" => "arm64-v8a",
            "armv7-linux-androideabi" => "armeabi-v7a",
            _ => panic!("Unsupported Android target: {}", target),
        };

        // Set up compiler paths
        let ndk_path = format!("{}/ndk/{}", android_home, ndk_version);
        let toolchain_path = format!("{}/toolchains/llvm/prebuilt/{}", ndk_path, host_tag);

        // Set environment variables for the cmake crate
        env::set_var("CC", format!("{}/bin/{}26-clang", toolchain_path, target));
        env::set_var(
            "CXX",
            format!("{}/bin/{}26-clang++", toolchain_path, target),
        );
        env::set_var("AR", format!("{}/bin/llvm-ar", toolchain_path));
        env::set_var("RANLIB", format!("{}/bin/llvm-ranlib", toolchain_path));

        dst = Config::new("../cpp")
            .generator("Ninja")
            .configure_arg("-DCMAKE_BUILD_TYPE=Release")
            .configure_arg(&format!("-DANDROID_ABI={}", android_abi))
            .configure_arg("-DANDROID_PLATFORM=android-33")
            .configure_arg(&format!(
                "-DCMAKE_TOOLCHAIN_FILE={}/build/cmake/android.toolchain.cmake",
                ndk_path
            ))
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

    // Link the `barretenberg` static library.
    println!("cargo:rustc-link-lib=static=barretenberg");

    // Link the C++ standard library.
    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }

    // Copy the headers to the build directory.
    // Fix an issue where the headers are not included in the build.
    Command::new("sh")
        .args(&[
            "copy-headers.sh",
            &format!("{}/build/include", dst.display()),
        ])
        .output()
        .unwrap();

    let mut builder = bindgen::Builder::default();

    if target_os == "android" {
        let android_home = option_env!("ANDROID_HOME").expect("ANDROID_HOME not set");
        let ndk_version = option_env!("NDK_VERSION").expect("NDK_VERSION not set");
        let host_tag = option_env!("HOST_TAG").expect("HOST_TAG not set");

        // Determine the target-specific include path
        let target_include = match target.as_str() {
            "x86_64-linux-android" => "x86_64-linux-android",
            "aarch64-linux-android" => "aarch64-linux-android",
            "armv7-linux-androideabi" => "arm-linux-androideabi",
            _ => panic!("Unsupported Android target: {}", target),
        };

        builder = builder
            // Add the include path for headers.
            .clang_args([
                "-std=c++20",
                "-xc++",
                &format!("-I{}/build/include", dst.display()),
                &format!(
                    "-I{}/ndk/{}/toolchains/llvm/prebuilt/{}/sysroot/usr/include/c++/v1",
                    android_home, ndk_version, host_tag
                ),
                &format!(
                    "-I{}/ndk/{}/toolchains/llvm/prebuilt/{}/sysroot/usr/include",
                    android_home, ndk_version, host_tag
                ),
                &format!(
                    "-I{}/ndk/{}/toolchains/llvm/prebuilt/{}/sysroot/usr/include/{}",
                    android_home, ndk_version, host_tag, target_include
                ),
            ]);
    } else if target_os == "ios" {
        // check which SDK we need
        let sdk = if target.contains("sim") || target.contains("x86_64") {
            "iphonesimulator"
        } else {
            "iphoneos"
        };
        let sdk_path = String::from_utf8(
            Command::new("xcrun")
                .args(&["--sdk", sdk, "--show-sdk-path"])
                .output()
                .expect("failed to get SDK path for iOS")
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string();

        // Build clang args based on target
        let mut clang_args = vec![
            "-std=c++20".to_string(),
            "-xc++".to_string(),
            format!("-I{}/build/include", dst.display()),
            "-isysroot".to_string(),
            sdk_path.clone(),
            "-target".to_string(),
            target.clone(),
        ];

        // Add minimum version based on target
        if target == "x86_64-apple-ios" {
            clang_args.push("-mios-simulator-version-min=13.0".to_string());
        } else if target == "aarch64-apple-ios-sim" {
            clang_args.push("-mios-simulator-version-min=14.0".to_string());
        } else if target == "aarch64-apple-ios" {
            clang_args.push("-miphoneos-version-min=15.0".to_string());
        }

        // Add include paths
        clang_args.push(format!("-I{}/usr/include/c++/v1", sdk_path));
        clang_args.push(format!("-I{}/usr/include", sdk_path));

        builder = builder
            .clang_args(&clang_args)
            // Ensure we're using the correct clang
            .clang_arg(&format!("--sysroot={}", sdk_path));
    } else if target_os == "macos" {
        builder = builder
            // Add the include path for headers.
            .clang_args([
                "-std=c++20",
                "-xc++",
                &format!("-I{}/build/include", dst.display()),
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
                #include <barretenberg/examples/simple/c_bind.hpp>
                #include <barretenberg/common/c_bind.hpp>
                #include <barretenberg/dsl/acir_proofs/c_bind.hpp>
            "#,
        )
        .allowlist_function("pedersen_commit")
        .allowlist_function("pedersen_hash")
        .allowlist_function("pedersen_hashes")
        .allowlist_function("pedersen_hash_buffer")
        .allowlist_function("poseidon_hash")
        .allowlist_function("poseidon_hashes")
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
        .allowlist_function("examples_simple_create_and_verify_proof")
        .allowlist_function("test_threads")
        .allowlist_function("common_init_slab_allocator")
        .allowlist_function("acir_get_circuit_sizes")
        .allowlist_function("acir_new_acir_composer")
        .allowlist_function("acir_delete_acir_composer")
        .allowlist_function("acir_init_proving_key")
        .allowlist_function("acir_create_proof")
        .allowlist_function("acir_load_verification_key")
        .allowlist_function("acir_init_verification_key")
        .allowlist_function("acir_get_verification_key")
        .allowlist_function("acir_get_proving_key")
        .allowlist_function("acir_verify_proof")
        .allowlist_function("acir_get_solidity_verifier")
        .allowlist_function("acir_serialize_proof_into_fields")
        .allowlist_function("acir_serialize_verification_key_into_fields")
        .allowlist_function("acir_prove_ultra_honk")
        .allowlist_function("acir_verify_ultra_honk")
        .allowlist_function("acir_write_vk_ultra_honk")
        .allowlist_function("acir_prove_and_verify_ultra_honk")
        .allowlist_function("acir_proof_as_fields_ultra_honk")
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
