import bson
from turkey import TurkeyMap

if __name__ == "__main__":
    db = TurkeyMap()
    name = bson.encode({"": "HelloWorld"})
    for i in range(10000):
        db.insert(i, "name", name)
        name_in_db = db.get(i, "name")
        (bson.decode(name_in_db))
