import pyo3_exception_testing

try:
    pyo3_exception_testing.add(1, 2)
except BaseException as e:
    if (
        e.__class__.__module__ == "pyo3_runtime"
        and e.__class__.__name__ == "PanicException"
    ):
        print(f"{e.__traceback__=}")
    raise (e)

print("Normal end")
