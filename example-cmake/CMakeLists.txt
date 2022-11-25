project(Line)

cmake_minimum_required(VERSION 3.5)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "-fcoroutines")

set(RUST_TARGET_DIR ${CMAKE_CURRENT_BINARY_DIR}/rust-line)

add_custom_target(
    line
    COMMAND cd ${CMAKE_SOURCE_DIR}/rust && cargo build
        --target-dir ${CMAKE_CURRENT_BINARY_DIR}/rust-line
)

add_executable(main cpp/main.cc)
add_dependencies(main line)
# Why are pthread and dl necessary? Who knows.
target_link_libraries(
    main
    ${RUST_TARGET_DIR}/debug/librust_line.a
    pthread
    dl
)
# This magically ensures that include paths work.
# You can try removing one of those lines and see what happens.
target_include_directories(
    main
    PRIVATE ${RUST_TARGET_DIR}/cxxbridge
    PRIVATE ${CMAKE_SOURCE_DIR}
)
