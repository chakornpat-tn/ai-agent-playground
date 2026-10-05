@echo off
setlocal

for /r src %%F in (*.rs) do (
  rustup run stable rustfmt --check --edition 2024 "%%F"
  if errorlevel 1 exit /b 1
)

for /r tests %%F in (*.rs) do (
  rustup run stable rustfmt --check --edition 2024 "%%F"
  if errorlevel 1 exit /b 1
)

echo Rust formatting check passed.
