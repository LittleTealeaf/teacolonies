from typing import List, Union, Dict
import nbtlib
import os
import shutil
import json
import hashlib
import zipfile



CRAFTERS = {
    "alchemist": ["crafting"],
    "baker": ["crafting", "smelting"],
    "blacksmith": ["crafting"],
    "chef": ["crafting", "smelting"],
    "concretemixer": ["custom"],
    "crusher": ["custom"],
    "dyer": ["crafting"],
    "enchanter": ["custom"],
    "farmer": ["crafting"],
    "fletcher": ["crafting"],
    "lumberjack": ["custom"],
    "mechanic": ["crafting"],
    "netherworker": ["custom"],
    "sifter": ["custom"],
    "stonemason": ["crafting"],
}


def dict_to_nbt(data):
    """
    Converts a Python dictionary to an nbtlib Compound.

    Args:
        data: The Python dictionary to convert.

    Returns:
        An nbtlib Compound object.
    """
    compound = nbtlib.Compound()
    for key, value in data.items():
        compound[key] = _convert_to_nbt(value)
    return compound


def _convert_to_nbt(value):
    """
    Converts a basic Python value to an nbtlib tag.

    Args:
        value: The Python value to convert.

    Returns:
        An nbtlib tag object.
    """
    if isinstance(value, dict):
        return dict_to_nbt(value)
    elif isinstance(value, list):
        return nbtlib.List([_convert_to_nbt(v) for v in value])
    elif isinstance(value, str):
        return nbtlib.String(value)
    elif isinstance(value, int):
        return nbtlib.Int(value)
    elif isinstance(value, float):
        return nbtlib.Float(value)
    elif isinstance(value, bool):
        return nbtlib.Byte(value)
    else:
        raise ValueError(f"Unsupported type: {type(value)}")


class Item:
    def __init__(self, name: str, count: int = 1, nbt: dict = None):
        if nbt is not None:
            tag = dict_to_nbt(nbt)
            serialized = nbtlib.serialize_tag(tag, compact=True)
            name = f"{name}{serialized}"
        self.name = name
        self.count = count

    def to_dict(self):
        if self.count == 1:
            return {"item": self.name}
        else:
            return {"item": self.name, "count": self.count}


class Pool:
    def __init__(
        self,
        entries: List[Union["Item", "LootTable"]],
        rolls: int = 1,
        bonus_rolls: float = 0.0,
        conditions: List[Dict] = None,
    ):
        self.entries = entries
        self.rolls = rolls
        self.bonus_rolls = bonus_rolls
        self.conditions = conditions if conditions is not None else []

    def to_dict(self):
        pool = {
            "rolls": self.rolls,
            "bonus_rolls": self.bonus_rolls,
            "entries": [self.entry_to_dict(e) for e in self.entries],
        }
        if self.conditions:
            pool["conditions"] = self.conditions
        return pool

    def entry_to_dict(self, entry):
        if isinstance(entry, Item):
            return {"type": "minecraft:item", "name": entry.name}
        elif isinstance(entry, LootTable):
            return {"type": "minecraft:loot_table", "name": entry.name}
        else:
            raise ValueError("Invalid entry type")


class LootTable:
    def __init__(self, pools: List[Pool] = None, functions: List[Dict] = None):
        self.pools = pools if pools is not None else []
        self.functions = functions if functions is not None else []
        self.name = None  # Name will be assigned by DataPack

    def to_dict(self):
        loot_table = {"pools": [p.to_dict() for p in self.pools]}
        if self.functions:
            loot_table["functions"] = self.functions
        return loot_table


class Hut:
    def __init__(
        self, datapack: "DataPack", building: str, crafting=False, custom=False
    ):
        self.datapack = datapack
        self.building = building
        self.features = {"craft": crafting, "custom": custom}

    def craft(
        self,
        inputs: List[Item] | Item,
        results: Item | List[Item],  # Item
        tool=None,
        intermediate=None,
        loot=None,
        additional_output: Item = None,
        min_building_level: int = None,
        max_building_level: int = None,
        research_id: str = None,
        not_research_id: str = None,
    ):
        if self.features["craft"]:
            return self.datapack.recipe(
                self.building,
                "crafting",
                inputs,
                results,
                tool,
                intermediate,
                loot,
                additional_output,
                min_building_level,
                max_building_level,
                research_id,
                not_research_id,
            )
        raise Exception("Hut does not support crafting")

    def custom(
        self,
        inputs: List[Item] | Item,
        results: Item | List[Item],
        tool=None,
        intermediate=None,
        loot=None,
        additional_output: Item = None,
        min_building_level: int = None,
        max_building_level: int = None,
        research_id: str = None,
        not_research_id: str = None,
    ):
        if self.features["custom"]:
            return self.datapack.recipe(
                self.building,
                "custom",
                inputs,
                results,
                tool,
                intermediate,
                loot,
                additional_output,
                min_building_level,
                max_building_level,
                research_id,
                not_research_id,
            )
        raise Exception("Hut does not support custom")


class DataPack:
    def __init__(self, description = ""):
        self.recipes = {}
        self.loottables = {}
        self.id_count = 0
        self.format = 18
        self.description = description

        self.sawmill = Hut(self, "sawmill", crafting = True)
        self.mechanic = Hut(self, "mechanic", crafting = True)
        self.lumberjack = Hut(self, "lumberjack", custom = True)
        self.blacksmith = Hut(self, "blacksmith", crafting = True)
        self.fletcher = Hut(self, "fletcher", crafting = True)
        self.builder = Hut(self, "builder", crafting= True)
        self.netherworker = Hut(self, "netherworker", custom = True)
        self.baker = Hut(self, "baker", crafting = True)
        self.farmer = Hut(self, "farmer", crafting = True)
        self.dyer = Hut(self, "dyer", crafting = True)
        self.sifter = Hut(self, "sifter", custom = True)
        self.stonemason = Hut(self, "stonemason", crafting = True)
        self.crusher = Hut(self, "crusher", custom = True)
        self.enchanter = Hut(self, "enchanter", custom = True)
        self.concretemixer = Hut(self, "concretemixer", custom = True)
        self.alchemist = Hut(self, "alchemist", crafting = True)
        self.chef = Hut(self, "chef", crafting = True)
        self.planter = Hut(self, "planter", crafting = True)



    def loot(self, loot_table: LootTable):
        loot_table.name = f"teacolonies:loot_tables/{self.id_count}"
        self.loottables[loot_table.name] = loot_table.to_dict()
        self.id_count += 1
        return loot_table.name

    def recipe(
        self,
        building: str,
        crafter: str,
        inputs: List[Item] | Item,
        results: Item | List[Item],  # Item
        tool=None,
        intermediate=None,
        loot=None,
        additional_output: Item = None,
        min_building_level: int = None,
        max_building_level: int = None,
        research_id: str = None,
        not_research_id: str = None,
    ):

        if not isinstance(inputs, list):
            inputs = [inputs]

        if not isinstance(results, list):
            results: List[Item] = [results]

        ids = []

        for result in results:
            id = str(self.id_count)
            self.id_count += 1

            rcp = {
                "type": "recipe",
                "crafter": f"{building}_{crafter}",
                "inputs": [i.to_dict() for i in inputs],
                "result": result.name,
                "intermediate": (
                    intermediate if intermediate is not None else "minecraft:air"
                ),
            }
            if result.count != 1:
                rcp["count"] = result.count
            if tool is not None:
                rcp["tool"] = tool
            if loot is not None:
                rcp["loot"] = loot
            if additional_output is not None:
                rcp["additional_output"] = additional_output.to_dict()
            if min_building_level is not None and min_building_level > 1:
                rcp["min_building_level"] = min_building_level
            if max_building_level is not None:
                rcp["max_building_level"] = max_building_level
            if research_id is not None:
                rcp["research_id"] = research_id
            if not_research_id is not None:
                rcp["not_research_id"] = not_research_id

            id = hashlib.md5(json.dumps(rcp, sort_keys=True).encode()).hexdigest()

            self.recipes[id] = rcp
            ids.append(f"teacolonies:recipes/{id}")
        return ids

    def save(self):
        with zipfile.ZipFile("teacolonies.zip", "w") as zipf:
            zipf.writestr("pack.mcmeta", json.dumps({
                "pack": {
                    "pack_format": self.format,
                    "description": self.description
                }
            }, indent = 4))
            count = 0

            for id, recipe in self.recipes.items():
                zipf.writestr(f"data/teacolonies/crafterrecipes/{id}.json", json.dumps(recipe, indent = 4))
                count += 1
            print(f"Saved {count} recipes")
