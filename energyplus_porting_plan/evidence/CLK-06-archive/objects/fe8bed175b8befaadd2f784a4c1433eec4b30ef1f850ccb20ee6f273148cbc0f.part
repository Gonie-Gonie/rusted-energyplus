# Test-only original-source driver. Inject into the existing EnergyPlus build with
# -DCMAKE_PROJECT_INCLUDE=<absolute path to this file>; no original source changes.
# DEFER runs after the original root has declared energypluslib and flag targets.
if(CMAKE_SOURCE_DIR STREQUAL PROJECT_SOURCE_DIR)
  get_property(_clk01_scheduled GLOBAL PROPERTY CLK01_REFERENCE_SCHEDULED)
  if(NOT _clk01_scheduled)
    set_property(GLOBAL PROPERTY CLK01_REFERENCE_SCHEDULED TRUE)
    set_property(GLOBAL PROPERTY CLK01_REFERENCE_SOURCE
      "${CMAKE_CURRENT_LIST_DIR}/clk01_reference.cpp")
    function(clk01_add_original_reference)
      get_property(_clk01_source GLOBAL PROPERTY CLK01_REFERENCE_SOURCE)
      if(NOT TARGET energypluslib)
        message(FATAL_ERROR "CLK-01 requires the original EnergyPlus energypluslib target")
      endif()
      add_executable(clk01_reference "${_clk01_source}")
      target_link_libraries(clk01_reference PRIVATE
        energypluslib project_options project_fp_options project_warnings)
      target_compile_features(clk01_reference PRIVATE cxx_std_20)
    endfunction()
    cmake_language(DEFER CALL clk01_add_original_reference)
  endif()
endif()
