swift_add_test(test-libsbp-cpp
    POST_BUILD
    SRCS
        test_sbp_stdio.cc
    INCLUDE
        ${PROJECT_SOURCE_DIR}/include/libsbp
    LINK
        sbp
        gtest_main)
swift_set_language_standards(test-libsbp-cpp)
swift_set_compile_options(test-libsbp-cpp)

file(COPY sbp_data/ DESTINATION ${CMAKE_CURRENT_BINARY_DIR})
