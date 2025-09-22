include(ExternalProject)

set(LMDB_PREFIX "${CMAKE_BINARY_DIR}/_deps/lmdb")
set(LMDB_INCLUDE "${LMDB_PREFIX}/src/lmdb_repo/libraries/liblmdb")
set(LMDB_LIB "${LMDB_INCLUDE}/liblmdb.a")
set(LMDB_HEADER "${LMDB_INCLUDE}/lmdb.h")
set(LMDB_OBJECT "${LMDB_INCLUDE}/*.o")

# Set up Android-specific build configuration if building for Android
if(ANDROID)
    # For Android NDK, we need to manually construct the sysroot path
    # The Android NDK toolchain file sets CMAKE_SYSROOT, so we can use that
    if(CMAKE_SYSROOT)
        set(ANDROID_SYSROOT_PATH "${CMAKE_SYSROOT}")
    else()
        # Fallback: construct path from ANDROID_NDK if available
        if(ANDROID_NDK)
            set(ANDROID_SYSROOT_PATH "${ANDROID_NDK}/toolchains/llvm/prebuilt/${ANDROID_HOST_TAG}/sysroot")
        else()
            message(FATAL_ERROR "Cannot determine Android sysroot path. CMAKE_SYSROOT not set and ANDROID_NDK not found.")
        endif()
    endif()

    # Set up architecture-specific include directory
    if(ANDROID_ABI STREQUAL "arm64-v8a")
        set(ANDROID_ARCH_INCLUDE "${ANDROID_SYSROOT_PATH}/usr/include/aarch64-linux-android")
    elseif(ANDROID_ABI STREQUAL "x86_64")
        set(ANDROID_ARCH_INCLUDE "${ANDROID_SYSROOT_PATH}/usr/include/x86_64-linux-android")
    elseif(ANDROID_ABI STREQUAL "armeabi-v7a")
        set(ANDROID_ARCH_INCLUDE "${ANDROID_SYSROOT_PATH}/usr/include/arm-linux-androideabi")
    elseif(ANDROID_ABI STREQUAL "x86")
        set(ANDROID_ARCH_INCLUDE "${ANDROID_SYSROOT_PATH}/usr/include/i686-linux-android")
    else()
        message(WARNING "Unknown Android ABI: ${ANDROID_ABI}, using default include path")
        set(ANDROID_ARCH_INCLUDE "${ANDROID_SYSROOT_PATH}/usr/include")
    endif()

    # Create Android-specific CFLAGS that include sysroot and system includes
    set(ANDROID_LMDB_CFLAGS "${CMAKE_C_FLAGS} --sysroot=${ANDROID_SYSROOT_PATH} -I${ANDROID_SYSROOT_PATH}/usr/include -I${ANDROID_ARCH_INCLUDE}")

    message(STATUS "Android LMDB build configuration:")
    message(STATUS "  Sysroot: ${ANDROID_SYSROOT_PATH}")
    message(STATUS "  Arch include: ${ANDROID_ARCH_INCLUDE}")
    message(STATUS "  CFLAGS: ${ANDROID_LMDB_CFLAGS}")

    ExternalProject_Add(
        lmdb_repo
        PREFIX ${LMDB_PREFIX}
        GIT_REPOSITORY "https://github.com/LMDB/lmdb.git"
        GIT_TAG ddd0a773e2f44d38e4e31ec9ed81af81f4e4ccbb
        BUILD_IN_SOURCE YES
        CONFIGURE_COMMAND "" # No configure step
        BUILD_COMMAND ${CMAKE_COMMAND} -E env
            CC=${CMAKE_C_COMPILER}
            CFLAGS=${ANDROID_LMDB_CFLAGS}
            make -C libraries/liblmdb -e XCFLAGS=-fPIC liblmdb.a
        INSTALL_COMMAND ""
        UPDATE_COMMAND "" # No update step
        BUILD_BYPRODUCTS ${LMDB_LIB}
    )
else()
    # Non-Android build
    ExternalProject_Add(
        lmdb_repo
        PREFIX ${LMDB_PREFIX}
        GIT_REPOSITORY "https://github.com/LMDB/lmdb.git"
        GIT_TAG ddd0a773e2f44d38e4e31ec9ed81af81f4e4ccbb
        BUILD_IN_SOURCE YES
        CONFIGURE_COMMAND "" # No configure step
        BUILD_COMMAND ${CMAKE_COMMAND} -E env
            CC=${CMAKE_C_COMPILER}
            CFLAGS=${CMAKE_C_FLAGS}
            make -C libraries/liblmdb -e XCFLAGS=-fPIC liblmdb.a
        INSTALL_COMMAND ""
        UPDATE_COMMAND "" # No update step
        BUILD_BYPRODUCTS ${LMDB_LIB}
    )
endif()

add_library(lmdb STATIC IMPORTED GLOBAL)
add_dependencies(lmdb lmdb_repo)
set_target_properties(lmdb PROPERTIES IMPORTED_LOCATION ${LMDB_LIB})

add_library(lmdb_objects OBJECT IMPORTED GLOBAL)
add_dependencies(lmdb_objects lmdb_repo)
set_target_properties(lmdb_objects PROPERTIES IMPORTED_LOCATION ${LMDB_OBJECT})
