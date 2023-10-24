from typing import Dict, NamedTuple


class TurkeyIndex(NamedTuple):
    ent_id: int
    prop_name: str


class TurkeyMap():
    def __init__(self):
        self.index_map: Dict[TurkeyIndex, bytes] = dict()
        self.id_map: Dict[int, bytes] = dict()
        self.prop_map: Dict[str, bytes] = dict()
        super().__init__()

    def insert(self, ent_id: int, prop_name: str, prop: bytes) -> None:
        self.index_map[TurkeyIndex(ent_id, prop_name)] = prop
        self.id_map[ent_id] = prop
        self.prop_map[prop_name] = prop

    def get(self, ent_id: int, prop_name: str) -> bytes:
        return self.index_map[TurkeyIndex(ent_id, prop_name)]
