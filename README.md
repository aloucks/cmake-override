# Override Cmake Generator Name

CMake will automatically default to generating MSVC build files when it detects
that Visual Studio is intalled. This can be problematic for building Rust
packages that have C dependencies that need to be built with MinGW.

This may be overcome by putting a batch file in the path before the
CMake's bin folder:

cmake.bat:
```batch
cmake.exe -G "MSYS Makefiles" %*
```

However, this doesn't work very well with an MSYS shell.

This ```cmake.exe``` will add "-G MSYS Makefiles" to the command line and
call the actual CMake executable.

The executable path may be explicitly set with the ```CMAKE_EXE_PATH```
environment variable.

The generator name may be set with the ```CMAKE_GEN_NAME``` environment
variable.

The following paths will be searched when executable is not specified via
environment variable:
- C:\Program Files\CMake\bin\cmake.exe
- C:\Program Files (x86)\CMake\bin\cmake.exe

Example:

```bash
$ export PATH=/c/path/to/cmake/override/bin:$PATH
$ export CMAKE_EXE_PATH=/c/CMake/bin/cmake.exe
$ export CMAKE_GEN_NAME="MSYS Makefiles"
$ cargo build myapp
```
