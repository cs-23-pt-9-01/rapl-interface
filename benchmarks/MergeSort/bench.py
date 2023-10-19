# NOTE MUST BE CALLED FROM ROOT

from ctypes import *
import sys
import platform

merge_param = sys.argv[1]
test_count =  int(sys.argv[2])
lib_path = "target\\release\\rapl_lib.dll" if platform.system() == "Windows" else "target/release/librapl_lib.so"

# test method
from heapq import merge

def merge_sort(m):
    if len(m) <= 1:
        return m

    middle = len(m) // 2
    left = m[:middle]
    right = m[middle:]

    left = merge_sort(left)
    right = merge_sort(right)
    return list(merge(left, right))

# start lib
dll = cdll.LoadLibrary(lib_path)

for i in range(test_count):
    # start recording
    dll.start_rapl()

    # run test
    result = merge_sort(merge_param)

    # stop recording
    dll.stop_rapl()
    print(result)

print("job done")
