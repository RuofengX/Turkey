import time

import bson

import native
from turkey import TurkeyMap


def benchmark_turkey():
    db = TurkeyMap()
    name = bson.dumps({"": "HelloWorld"})
    start_time = time.time()
    for i in range(100000):
        for j in range(1000):
            db.insert(i, f"name{j}", name)

    for i in range(1000):
        name_in_db = db.get(i, f"name{i}")
        if name_in_db is None:
            print("err")
        else:
            (bson.loads(name_in_db))
    print(time.time() - start_time)


def benchmark_native():
    db = native.TurkeyMap()
    name = bson.dumps({"": "HelloWorld"})
    start_time = time.time()
    for i in range(100000):
        for j in range(1000):
            db.insert(i, f"name{j}", name)

    for i in range(1000):
        name_in_db = db.get(i, f"name{i}")
        if name_in_db is not None:
            (bson.loads(name_in_db))
        else:
            print("err")
    print(time.time() - start_time)


if __name__ == "__main__":
    benchmark_turkey()
    benchmark_native()
