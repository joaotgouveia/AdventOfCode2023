add_executable(day05p1 day05p1.cpp)
add_executable(day05p2 day05p2.cpp)

if(CLANG_TIDY_COMMAND)
    set_target_properties(day05p1 PROPERTIES CXX_CLANG_TIDY "${CLANG_TIDY_COMMAND}")
    set_target_properties(day05p2 PROPERTIES CXX_CLANG_TIDY "${CLANG_TIDY_COMMAND}")
endif()
if(CPPCHECK_COMMAND)
    set_target_properties(day05p1 PROPERTIES CXX_CPPCHECK "${CPPCHECK_COMMAND}")
    set_target_properties(day05p2 PROPERTIES CXX_CPPCHECK "${CPPCHECK_COMMAND}")
endif()
