@echo off

cd %~dp0\api
python gen-api.py
cd ..

set CARGO_HOME=%USERPROFILE%/.cargo
if not exist "%CARGO_HOME%\lib\azul-dll-0.0.1\target\release" mkdir "%CARGO_HOME%\lib\azul-dll-0.0.1\target\release"

cd "%~dp0\azul-dll"
taskkill /im "cargo.exe"
cargo build --all-features --release
rem RUSTFLAGS='-C link-arg=-s'
rem cargo build --all-features
rem cargo install --path .
cd ..

copy "%~dp0\target\release\libazul.so" "%CARGO_HOME%\lib\azul-dll-0.0.1\target\release"

if exist "%~dp0\target\debug\examples" del "%~dp0\target\debug\examples\azul.dll"

RUST_BACKTRACE=full cargo run --example public

pause >nul