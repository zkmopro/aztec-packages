if(NOT DISABLE_AZTEC_VM)
  barretenberg_module(vm2 sumcheck stdlib_honk_verifier stdlib_goblin_verifier)

  # Disable sign-conversion error for this specific comparison issue in verifier.cpp
  if(CMAKE_CXX_COMPILER_ID MATCHES "GNU")
    target_compile_options(vm2_objects PRIVATE -Wno-error=sign-compare)
  elseif(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
    target_compile_options(vm2_objects PRIVATE -Wno-error=sign-compare)
  endif()
endif()
