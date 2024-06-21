import ctypes
import time

def python_counter(n):
    start = 0
    while start < n:
        start += 1
    return start

rust_lib_path = "/home/dmitriy.sinitskiy/Documents/otus_rust/rust_py/target/release/deps/librust_py.so"
rustlib = ctypes.CDLL(rust_lib_path)


start = time.time()
python_result = python_counter(1_000_000)
stop = time.time()
print(f"py_counter. time: {start-stop}. result {python_result}")


start = time.time()
rust_result = rustlib.rust_counter(1_000_000)
stop = time.time()
print(f"rust_counter. time: {start-stop}. result {rust_result}")
