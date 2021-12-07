cmake_minimum_required(VERSION 3.0)

project(grade_calculator)

add_executable(grade_calculator main.cpp)

install(TARGETS grade_calculator RUNTIME DESTINATION bin)
