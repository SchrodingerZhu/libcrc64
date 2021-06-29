project(crc64)
cmake_minimum_required(VERSION 3.0)
find_package(Threads REQUIRED)
add_subdirectory(corrosion)

include_directories(include)
corrosion_import_crate(MANIFEST_PATH Cargo.toml)
target_link_libraries(crc64 INTERFACE ${CMAKE_DL_LIBS} Threads::Threads)

option(CRC64_ENABLE_TESTS "Enable Tests" ON)

if (CRC64_ENABLE_TESTS)
    enable_testing()
    add_executable(ffi src/test.c)
    add_test(test_ffi ffi)
    target_link_libraries(ffi crc64)
endif()