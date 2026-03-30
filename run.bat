cls

echo "[MSG] Prefer using cargo instead of scripts."

cd src
rustc main.rs

move main.exe ..\out

cd ..
.\out\main.exe

del out
