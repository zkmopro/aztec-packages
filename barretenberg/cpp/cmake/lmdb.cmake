include(ExternalProject)

set(LMDB_PREFIX "${CMAKE_BINARY_DIR}/_deps/lmdb")
set(LMDB_INCLUDE "${LMDB_PREFIX}/src/lmdb_repo/libraries/liblmdb")
set(LMDB_LIB "${LMDB_INCLUDE}/liblmdb.a")
set(LMDB_HEADER "${LMDB_INCLUDE}/lmdb.h")
set(LMDB_OBJECT "${LMDB_INCLUDE}/*.o")

# Construct CFLAGS with proper sysroot and target handling
# Check if CMAKE_C_FLAGS already contains -isysroot or --sysroot (from toolchain files)
# This handles cases where toolchains add sysroot (iOS simulator) vs where they don't (iOS device)
set(LMDB_CFLAGS "${CMAKE_C_FLAGS}")
string(FIND "${CMAKE_C_FLAGS}" "-isysroot" HAS_ISYSROOT)
string(FIND "${CMAKE_C_FLAGS}" "--sysroot" HAS_SYSROOT)
string(FIND "${CMAKE_C_FLAGS}" "--target" HAS_TARGET)

if(HAS_ISYSROOT EQUAL -1 AND HAS_SYSROOT EQUAL -1)
    # Only add sysroot if not already present in CMAKE_C_FLAGS
    if(CMAKE_OSX_SYSROOT)
        # For iOS/macOS, use -isysroot flag with the sysroot path
        set(LMDB_CFLAGS "${LMDB_CFLAGS} -isysroot ${CMAKE_OSX_SYSROOT}")
    elseif(CMAKE_SYSROOT)
        # For Android/Linux cross-compilation, use --sysroot flag
        set(LMDB_CFLAGS "${LMDB_CFLAGS} --sysroot=${CMAKE_SYSROOT}")
    endif()
endif()

# For cross-compilation, add --target if not already present
if(HAS_TARGET EQUAL -1 AND CMAKE_C_COMPILER_TARGET)
    set(LMDB_CFLAGS "${LMDB_CFLAGS} --target=${CMAKE_C_COMPILER_TARGET}")
endif()

ExternalProject_Add(
    lmdb_repo
    PREFIX ${LMDB_PREFIX}
    GIT_REPOSITORY "https://github.com/LMDB/lmdb.git"
    GIT_TAG ddd0a773e2f44d38e4e31ec9ed81af81f4e4ccbb
    BUILD_IN_SOURCE YES
    CONFIGURE_COMMAND "" # No configure step
    BUILD_COMMAND ${CMAKE_COMMAND} -E env
        CC=${CMAKE_C_COMPILER}
        "CFLAGS=${LMDB_CFLAGS}"
        make -C libraries/liblmdb -e XCFLAGS=-fPIC liblmdb.a
    INSTALL_COMMAND ""
    UPDATE_COMMAND "" # No update step
    BUILD_BYPRODUCTS ${LMDB_LIB}
)

add_library(lmdb STATIC IMPORTED GLOBAL)
add_dependencies(lmdb lmdb_repo)
set_target_properties(lmdb PROPERTIES IMPORTED_LOCATION ${LMDB_LIB})

add_library(lmdb_objects OBJECT IMPORTED GLOBAL)
add_dependencies(lmdb_objects lmdb_repo)
set_target_properties(lmdb_objects PROPERTIES IMPORTED_LOCATION ${LMDB_OBJECT})
