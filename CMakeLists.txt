project(crc64)
cmake_minimum_required(VERSION 3.0)

add_subdirectory(corrosion)

include_directories(include)
corrosion_import_crate(MANIFEST_PATH Cargo.toml)