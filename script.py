from lib import DataPack, Item, Pool, LootTable


def m(text):
    return f"minecraft:{text}"


def mc(text):
    return f"minecolonies:{text}"


def do(text):
    return f"domum_ornamentum:{text}"


dp = DataPack()

# CONFIGURATION:
POTION_STRONG_MIN_LEVEL = 2
POTION_LONG_MIN_LEVEL = 4
POTION_SPLASH_MIN_LEVEL = 3
POTION_LINGERING_MIN_LEVEL = 5


### TREES

TREES = [
    "cherry",
    "acacia",
    "birch",
    "cherry",
    "dark_oak",
    "jungle",
    "oak",
    "spruce",
    "mangrove",
    "crimson",
    "warped",
    "bamboo",
]


# Leaves / Saplings
for tree in [
    "cherry",
    "acacia",
    "birch",
    "cherry",
    "dark_oak",
    "jungle",
    "oak",
    "spruce",
]:
    sapling = m(f"{tree}_sapling")
    log = m(f"{tree}_log")
    leaves = m(f"{tree}_leaves")

    dp.lumberjack.custom(Item(mc("compost")), Item(sapling, 4))
    dp.lumberjack.custom(
        [Item(sapling), Item(mc("compost"))], Item(log, 4), min_building_level=5
    )
    dp.lumberjack.custom(Item(sapling), Item(leaves))
    dp.lumberjack.custom(Item(leaves), Item(m("flowering_azalea")))
    dp.alchemist.craft(Item(leaves), Item(mc("mistletoe"), 3), tool=m("shears"))

dp.planter.craft(Item(m("bamboo"), 9), Item(m("bamboo_block")))

for tree in TREES:
    planks = m(f"{tree}_planks")
    log = m(f"{tree}_log")
    stripped_log = m(f"stripped_{tree}_{'log' if tree != 'bamboo' else 'block'}")
    slab = m(f"{tree}_slab")

    if tree == "bamboo":
        log = m("bamboo_block")
        stripped_log = m("stripped_bamboo_block")
    if tree == "crimson":
        log = m("crimson_stem")
        stripped_log = m("stripped_crimson_stem")
    if tree == "warped":
        log = m("warped_stem")
        stripped_log = m("stripped_warped_stem")

    dp.sawmill.craft(Item(log), Item(planks, 2 if tree == "bamboo" else 4))
    dp.sawmill.craft(Item(planks, 3), [Item(slab, 6), Item(m("bowl"), 4)])
    dp.sawmill.craft(
        Item(planks, 6),
        [
            Item(m(f"{tree}_stairs"), 4),
            Item(m(f"{tree}_door"), 3),
            Item(m(f"{tree}_trapdoor"), 2),
        ],
    )

    dp.sawmill.craft(
        [Item(log, 3), Item(m("stick"), 3), Item(m("charcoal"))], Item(m("campfire"))
    )

    dp.sawmill.craft(
        [Item(planks, 4), Item(m("stick"), 2)], Item(m(f"{tree}_fence"), 3)
    )
    dp.sawmill.craft(
        [Item(planks, 2), Item(m("stick"), 4)], Item(m(f"{tree}_fence_gate"), 3)
    )
    dp.sawmill.craft([Item(planks, 6), Item(m("stick"), 1)], Item(m(f"{tree}_sign"), 3))
    dp.sawmill.craft(Item(planks, 2), Item(m("stick"), 4))
    dp.mechanic.craft(Item(planks), Item(m(f"{tree}_button")))
    dp.sawmill.craft(Item(planks, 4), Item(m("crafting_table")))
    dp.sawmill.craft(Item(planks, 8), Item(m("chest")))
    dp.sawmill.craft([Item(planks, 6), Item(slab, 2)], Item(m("barrel")))
    dp.sawmill.craft([Item(m("white_wool"), 3), Item(planks, 3)], Item(m("white_bed")))
    dp.sawmill.craft(
        [Item(m("stick"), 6), Item(planks, 3)],
        [
            Item(do("blockbarreldeco_onside")),
            Item(do("blockbarreldeco_standing")),
        ],
    )
    dp.sawmill.craft(
        [Item(planks, 6), Item(m("iron_nugget"), 2)], Item(mc("blockminecoloniesrack"))
    )
    dp.sawmill.craft([Item(planks, 6), Item(m("honeycomb"), 3)], Item(m("beehive")))
    dp.sawmill.craft([Item(planks, 6), Item(m("chest"), 2)], Item(mc("blockstash")))
    dp.sawmill.craft(
        [Item(planks, 6), Item(m("iron_ingot")), Item(m("dirt"))],
        Item(mc("barrel_block")),
    )
    dp.sawmill.craft(
        [Item(planks, 2), Item(m("stick"), 2), Item(m("stone_slab"))],
        Item(m("grindstone")),
    )

    dp.sawmill.craft(
        [Item(planks, 3), Item(m("stick"), 2)],
        [Item(m("wooden_axe")), Item(m("wooden_pickaxe"))],
    )
    dp.sawmill.craft([Item(planks, 1), Item(m("stick"), 2)], Item(m("wooden_shovel")))
    dp.sawmill.craft([Item(planks, 2), Item(m("stick"), 2)], Item(m("wooden_hoe")))
    dp.sawmill.craft([Item(planks, 2), Item(m("stick"), 1)], Item(m("wooden_sword")))
    dp.blacksmith.craft([Item(planks, 6), Item(m("iron_ingot"))], Item(m("shield")))
    dp.blacksmith.craft(
        [Item(planks, 2), Item(m("iron_ingot"), 2)], Item(m("smithing_table"))
    )
    dp.sawmill.craft([Item(log, 4), Item(m("furnace"))], Item(m("smoker")))
    dp.sawmill.craft(
        [Item(planks, 6), Item(m(f"{tree}_slab"), 3)], Item(m("chiseled_bookshelf"))
    )
    dp.sawmill.craft(
        [Item(stripped_log, 6), Item(m("chain"), 2)], Item(m(f"{tree}_hanging_sign"), 6)
    )

    # Hut Blocks
    dp.sawmill.craft([Item(planks, 7), Item(m("string"))], Item(mc("blockhutfletcher")))
    dp.sawmill.craft(
        [Item(planks, 7), Item(m("egg"))], Item(mc("blockhutchickenherder"))
    )
    dp.sawmill.craft(
        [Item(planks, 6), Item(m("book"), 2)], Item(mc("blockhutuniversity"))
    )
    dp.sawmill.craft(
        [Item(planks, 6), Item(m(f"{tree}_door"))], Item(mc("blockhutbuilder"))
    )
    dp.mechanic.craft(Item(planks, 2), Item(m(f"{tree}_pressure_plate")))


# Dead Bush
dp.lumberjack.custom(Item(m("stick"), 5), Item(m("dead_bush")))

dp.fletcher.craft([Item(m("leather")), Item(m("string"))], Item(m("name_tag")))

## Netherite Stuff

dp.blacksmith.craft(
    Item(m("smithing_table")),
    Item(m("netherite_upgrade_smithing_template")),
    min_building_level=5,
)
dp.blacksmith.craft(
    [Item(m("netherite_scrap"), 4), Item(m("gold_ingot"), 4)],
    Item(m("netherite_ingot")),
    min_building_level=5,
)
dp.blacksmith.craft(
    [Item(m("blackstone"), 4), Item(m("netherrack"), 4)],
    Item(m("netherite_scrap")),
    min_building_level=5,
)


# Ore Weapon/Armor Crafting
def ore_craft(prefix, resource, level):
    dp.blacksmith.craft(
        Item(resource, 4), Item(m(f"{prefix}_boots")), min_building_level=level
    )
    dp.blacksmith.craft(
        Item(resource, 7), Item(m(f"{prefix}_leggings")), min_building_level=level
    )
    dp.blacksmith.craft(
        Item(resource, 8), Item(m(f"{prefix}_chestplate")), min_building_level=level
    )
    dp.blacksmith.craft(
        Item(resource, 5), Item(m(f"{prefix}_helmet")), min_building_level=level
    )
    dp.blacksmith.craft(
        [Item(resource, 3), Item(m("stick"), 2)],
        [Item(m(f"{prefix}_pickaxe")), Item(m(f"{prefix}_axe"))],
        min_building_level=level,
    )
    dp.blacksmith.craft(
        [Item(resource, 1), Item(m("stick"), 2)],
        Item(m(f"{prefix}_shovel")),
        min_building_level=level,
    )
    dp.blacksmith.craft(
        [Item(resource, 2), Item(m("stick"), 2)],
        Item(m(f"{prefix}_hoe")),
        min_building_level=level,
    )
    dp.blacksmith.craft(
        [Item(resource, 2), Item(m("stick"), 1)],
        Item(m(f"{prefix}_sword")),
        min_building_level=level,
    )


ore_craft("iron", "iron_ingot", 3)
ore_craft("golden", "gold_ingot", 2)
ore_craft("diamond", "diamond", 4)
del ore_craft

dp.blacksmith.craft(
    [Item(m("iron_nugget"), 5), Item(m("iron_ingot"))], Item(m("chainmail_helmet"))
)
dp.blacksmith.craft(
    [Item(m("iron_nugget"), 6), Item(m("iron_ingot"), 2)],
    Item(m("chainmail_chestplate")),
)
dp.blacksmith.craft(
    [Item(m("iron_nugget"), 4), Item(m("iron_ingot"), 3)], Item(m("chainmail_leggings"))
)
dp.blacksmith.craft(
    [Item(m("iron_nugget"), 2), Item(m("iron_ingot"), 2)], Item(m("chainmail_boots"))
)

for stone in [m("cobblestone"), m("cobbled_deepslate"), m("blackstone")]:
    dp.blacksmith.craft(
        [Item(stone, 3), Item(m("stick"), 2)],
        [Item(m("stone_pickaxe")), Item(m("stone_axe"))],
    )
    dp.blacksmith.craft([Item(stone, 1), Item(m("stick"), 2)], Item(m("stone_shovel")))
    dp.blacksmith.craft([Item(stone, 2), Item(m("stick"), 2)], Item(m("stone_hoe")))
    dp.blacksmith.craft([Item(stone, 2), Item(m("stick"), 1)], Item(m("stone_sword")))


### Mechanic

# Quartz Block
dp.mechanic.craft(Item(m("quartz"), 4), Item(m("quartz_block")))

# Slime Ball from clay and honey
dp.mechanic.craft(
    [Item(m("clay_ball"), 8), Item(m("honey_bottle"))],
    Item(m("slime_ball"), 8),
    additional_output=Item(m("glass_bottle")),
)

dp.mechanic.craft(
    [Item(m("water_bucket"), 1), Item(m("lava_bucket"), 1)],
    Item(m("cobblestone"), 16),
    tool=mc("pickaxe"),
    additional_output=Item(m("bucket"), 2),
)

dp.mechanic.craft(
    [Item(m("water_bucket"), 1)], Item(m("ice"), 4), additional_output=Item(m("bucket"))
)


# Coppers
for c in ["", "_cut"]:
    dp.mechanic.craft(
        [Item("minecraft:copper_block" if c == "" else "minecraft:cut_copper")],
        Item(f"minecraft:exposed{c}_copper"),
    )
    for f, t in [("exposed", "weathered"), ("weathered", "oxidized")]:
        dp.mechanic.craft(
            [Item(f"minecraft:{f}{c}_copper")],
            Item(f"minecraft:{t}{c}_copper"),
        )

# Raw Ore Blocks
for ore in ["gold", "iron", "copper"]:
    dp.mechanic.craft(
        [Item(f"minecraft:{ore}_ingot", 6)],
        Item(f"minecraft:raw_{ore}_block"),
    )

# Cobblestone
dp.mechanic.craft(
    [Item("minecraft:water_bucket"), Item("minecraft:lava_bucket")],
    Item("minecraft:cobblestone", 6),
    additional_output=Item("minecraft:bucket", 2),
)


## Builder to make ores
def builder_ore(prefix, resource=None):
    resource = resource if resource is not None else prefix
    for deep_prefix, stone_res in [("", "stone"), ("deepslate_", "deepslate")]:
        dp.builder.craft(
            [Item(m(stone_res), 3), Item(m(resource), 2)],
            Item(m(f"{deep_prefix}{prefix}_ore"), 2),
        )


[builder_ore(i) for i in ["emerald", "diamond", "redstone", "coal"]]
builder_ore("gold", "gold_ingot")
builder_ore("lapis", "lapis_lazuli")
builder_ore("iron", "iron_ore")
builder_ore("copper", "copper_ore")


### Netherworker

# Netherrack
dp.netherworker.custom([Item(m("stone_pickaxe"))], Item(m("netherrack"), 32))
dp.netherworker.custom([Item(m("iron_pickaxe"))], Item(m("netherrack"), 64))

# Nether Items
for variant, mushroom in [("crimson", "red"), ("warped", "brown")]:
    dp.netherworker.custom(
        [Item("minecraft:netherrack"), Item(f"minecraft:{mushroom}_mushroom", 4)],
        Item(f"minecraft:{variant}_fungus", 4),
    )
    dp.netherworker.custom(
        [Item(m("netherrack")), Item(m(f"{variant}_fungus"))],
        Item(m(f"{variant}_nylium")),
    )
    dp.netherworker.custom(
        [Item(m(f"{variant}_nylium"))],
        Item(m(f"{variant}_roots")),
        tool=m("shears"),
        additional_output=Item(m(f"{variant}_nylium")),
    )

dp.netherworker.custom(
    [Item(m("lava_bucket")), Item(m("deepslate"), 4)],
    Item(m("basalt"), 4),
    additional_output=Item(m("bucket")),
)

dp.netherworker.custom(
    [Item(m("lava_bucket"), 8), Item(m("water_bucket"))],
    Item(m("obsidian"), 8),
    tool=mc("pickaxe"),
    additional_output=Item(m("bucket"), 9),
)

dp.netherworker.custom(
    [Item("minecraft:glowstone"), Item("minecraft:mycelium")],
    [
        Item("minecraft:shroomlight"),
        Item("minecraft:pearlescent_froglight"),
        Item("minecraft:verdant_froglight"),
        Item("minecraft:ochre_froglight"),
    ],
)

dp.netherworker.custom(
    [Item(m("lava_bucket")), Item(m("netherrack"), 4)],
    Item(m("blaze_rod")),
    additional_output=Item(m("bucket")),
)


### Baker

dp.baker.craft(Item(m("bucket")), Item(m("water_bucket")))

for fish in ["cod", "salmon", "tropical_fish", "pufferfish"]:
    dp.baker.craft(
        [Item(m("water_bucket")), Item(m(fish))],
        Item(m(f"{fish}_bucket")),
    )


### Farmer
# Dirt
dp.farmer.craft(Item(mc("compost"), 4), Item(m("dirt"), 4))

# Mushrooms
dp.farmer.craft(
    Item(mc("composted_dirt")),
    [Item(m("brown_mushroom"), 8), Item(m("red_mushroom"), 8)],
    additional_output=Item(mc("composted_dirt")),
)

# Grass
dp.farmer.craft(
    [Item(m("dirt")), Item(m("wheat_seeds"))],
    [
        Item(m("grass_block")),
        Item(m("tall_grass"), 4),
        Item(m("grass"), 8),
        Item(m("large_fern"), 4),
    ],
    tool=m("shears"),
)

# Mycelium
dp.farmer.craft(
    [
        Item(m("grass_block")),
        Item(m("red_mushroom")),
        Item(m("brown_mushroom")),
    ],
    Item(m("mycelium"), 1),
)

# Rooted Dirt
dp.farmer.craft([Item(m("dirt"), 4)], Item(m("rooted_dirt"), 2))
dp.farmer.craft(Item(m("pumpkin")), Item(m("pumpkin_seeds"), 4))


### Fletcher

# Cobweb
dp.fletcher.craft([Item(m("string"), 3)], Item(m("cobweb")))

# Moss Block
dp.fletcher.craft([Item(m("vine"), 4)], Item(m("moss_block"), 2))

dp.fletcher.craft(Item(m("rotten_flesh"), 4), Item(m("leather"), 3))


## Dyer (This is all inclusive)


# First we get the dye crafting recipes themselves


def s(i, count=1):
    return (i, count)


COLORS = {
    "white": [s(Item(m("bone_meal"))), s(Item(m("lily_of_the_valley")))],
    "light_gray": [
        s(Item(m("oxeye_daisy"))),
        s(Item(m("azure_bluet"))),
        s(Item(m("white_tulip"))),
        s([Item(m("black_dye")), Item(m("white_dye"), 2)], 3),
        s([Item(m("gray_dye")), Item(m("white_dye"))], 2),
    ],
    "light_blue": [
        s(Item(m("blue_orchid"))),
        s([Item(m("blue_dye")), Item(m("white_dye"))]),
    ],
    "gray": [s([Item(m("white_dye")), Item(m("black_dye"))], 2)],
    "black": [s(Item(m("ink_sac"))), s(Item(m("wither_rose")))],
    "brown": [s(Item(m("cocoa_beans")))],
    "red": [
        s(Item(m("fire_coral_block")), 16),
        s(Item(m("red_tulip"))),
        s(Item(m("beetroot"))),
        s(Item(m("poppy"))),
        s(Item(m("rose_bush")), 2),
    ],
    "orange": [
        s(Item(m("torchflower"))),
        s(Item(m("orange_tulip"))),
        s([Item(m("red_dye")), Item(m("yellow_dye"))], 2),
    ],
    "yellow": [
        s(Item(m("dandelion"))),
        s(Item(m("sunflower")), 2),
        s(Item(m("horn_coral_block")), 16),
    ],
    "lime": [s([Item(m("green_dye")), Item(m("white_dye"))], 2)],
    "green": [s(Item(m("kelp")))],
    "cyan": [
        s(Item(m("pitcher_plant")), 2),
        s([Item(m("blue_dye")), Item(m("green_dye"))], 2),
    ],
    "blue": [
        s(Item(m("cornflower"))),
        s(Item(m("tube_coral_block")), 16),
        s(Item(m("lapis_lazuli"))),
    ],
    "purple": [s([Item(m("blue_dye")), Item(m("red_dye"))], 2)],
    "magenta": [
        s([Item(m("blue_dye")), Item(m("red_dye"), 2), Item(m("white_dye"))], 4),
        s(Item(m("allium"))),
        s([Item(m("blue_dye")), Item(m("red_dye")), Item(m("pink_dye"))], 3),
        s(Item(m("lilac")), 2),
        s([Item(m("purple_dye")), Item(m("pink_dye"))], 2),
        s(Item(m("bubble_coral_block")), 16),
    ],
    "pink": [
        s([Item(m("red_dye")), Item(m("white_dye"))], 2),
        s(Item(m("brain_coral_block")), 16),
        s(Item(m("pink_petals"))),
        s(Item(m("pink_tulip"))),
        s(Item(m("peony"))),
    ],
}

for color, recipes in COLORS.items():
    for recipe, count in recipes:
        dp.dyer.craft(recipe, Item(m(f"{color}_dye"), count))

del s

for color in COLORS.keys():
    dye = m(f"{color}_dye")

    dp.fletcher.craft([Item(m(f"{color}_wool"), 2)], Item(mc("colony_banner")))

    for block, count in [("candle", 1), ("terracotta", 8), ("shulker_box", 1)]:
        dp.dyer.craft(
            [Item(dye), Item(m(block), count)], Item(m(f"{color}_{block}"), count)
        )

    # white <--> other colors
    if color != "white":
        for block, count in [("wool", 4), ("bed", 1), ("carpet", 2)]:
            dp.dyer.craft(
                [Item(dye), Item(m(f"white_{block}"), count)],
                Item(m(f"{color}_{block}"), count),
            )
            # dp.dyer.craft(
            #     [
            #         Item(m("water_bucket")),
            #         Item(m(f"{color}_{block}"), count * 2),
            #     ],
            #     Item(m(f"white_{block}"), count * 2),
            #     additional_output=Item(m("water_bucket")),
            # )

    # Glass
    for base, style in [
        (m("glass_pane"), "_stained_glass_pane"),
        (m("glass"), "_stained_glass"),
    ]:
        dp.dyer.craft([Item(dye), Item(base, 8)], Item(m(f"{color}{style}"), 8))

    dp.dyer.craft(
        [Item(dye), Item(m("brick"), 4)],
        Item(do(f"{color}_brick_extra")),
    )

    # Banner
    dp.fletcher.craft(
        [Item(m(f"{color}_wool"), 6), Item(m("stick"))],
        Item(m(f"{color}_banner")),
    )

    # Floating Carpet
    dp.fletcher.craft(
        [Item(m(f"{color}_carpet")), Item(m("string"))],
        Item(do(f"{color}_floating_carpet")),
    )


# Bell
dp.mechanic.craft([Item(m("gold_ingot"), 4), Item(m("iron_ingot"))], Item(m("bell")))


### Stonework Slabs and Stairs
def stonework(base: str, prefix: str = None, wall: bool = False):
    if prefix is None:
        prefix = base
    ## Slab
    dp.stonemason.craft(Item(m(base), 3), Item(m(f"{prefix}_slab"), 6))
    ## Stairs
    dp.stonemason.craft(Item(m(base), 6), Item(m(f"{prefix}_stairs"), 4))

    if wall:
        dp.stonemason.craft(Item(m(base), 6), Item(m(f"{prefix}_wall"), 6))


SIMPLES = ["stone", "mossy_cobblestone", "smooth_stone", "smooth_quartz"]
for waxed in ["", "waxed_"]:
    SIMPLES += [
        f"{waxed}{pre}cut_copper" for pre in ["", "exposed_", "weathered_", "oxidized_"]
    ]

for simple in SIMPLES:
    stonework(simple)

del SIMPLES

stonework("cobblestone", wall=True)
stonework("cobbled_deepslate", wall=True)
stonework("bricks", prefix="brick", wall=True)
stonework("stone_bricks", prefix="stone_brick", wall=True)
stonework("mossy_stone_bricks", prefix="mossy_stone_brick", wall=True)
stonework("deepslate_tiles", prefix="deepslate_tile", wall=True)
stonework("end_stone_bricks", prefix="end_stone_brick", wall=True)
stonework("purpur_block", prefix="purpur")
stonework("quartz_block", prefix="quartz")

for p in ["polished_", ""]:
    for stone in ["granite", "andesite", "diorite", "blackstone"]:
        stonework(f"{p}{stone}", wall=p == "" or stone == "blackstone")


# Alchemist
dp.alchemist.craft(Item(m("ender_pearl"), 16), Item(m("nether_star")))
dp.alchemist.craft(Item(m("glass_bottle")), Item(m("potion"), nbt={"Potion": "water"}))
dp.alchemist.craft(
    [Item(mc("mistletoe")), Item(mc("large_water_bottle"))], Item(mc("magicpotion"))
)
dp.alchemist.craft(
    [Item(m("gold_nugget"), 8), Item(m("carrot"))], Item(m("golden_carrot"))
)
dp.alchemist.craft(
    [Item(m("lava_bucket")), Item(m("glass_bottle"), 8)], Item(m("dragon_breath"), 2)
)
dp.alchemist.craft(
    [Item(m("gold_nugget"), 8), Item(m("melon_slice"))],
    Item(m("glistering_melon_slice")),
)


POTION_NAMES = []


def brew_craft(
    from_potion_item, from_potion, ingredient, to_potion_item, to_potion, min_level=1
):
    # dp.recipe("alchemist", "brewing", [Item(ingredient)] + ([Item(f"minecraft:{from_potion_item}", 1, {"Potion": from_potion})] * 3), Item(f"minecraft:{to_potion_item}", 3, {"Potion": to_potion}), min_building_level = min_level, intermediate="minecraft:brewing_stand")

    # Crafting Method
    # dp.alchemist.craft(
    #     [Item(ingredient), Item("minecraft:blaze_powder")]
    #     + ([Item(f"minecraft:{from_potion_item}", 1, {"Potion": from_potion})] * 3),
    #     Item(f"minecraft:{to_potion_item}", 3, {"Potion": to_potion}),
    #     min_building_level=min_level,
    # )
    dp.alchemist.craft(
        [
            Item(ingredient),
            Item(m("blaze_powder")),
            Item(m(from_potion_item), 3, {"Potion": from_potion}),
        ],
        Item(m(to_potion_item), 3, {"Potion": to_potion}),
        min_building_level=min_level,
    )


def brew(
    from_potion,
    ingredient,
    to_potion,
    strong=False,
    long=False,
    splash=True,
    lingering=True,
    min_level=1,
):
    if from_potion not in POTION_NAMES:
        POTION_NAMES.append(from_potion)
    if to_potion not in POTION_NAMES:
        POTION_NAMES.append(to_potion)

    brew_craft("potion", from_potion, ingredient, "potion", to_potion, min_level)
    if splash:
        brew_craft(
            "splash_potion",
            from_potion,
            ingredient,
            "splash_potion",
            to_potion,
            min_level=max(min_level, POTION_SPLASH_MIN_LEVEL),
        )
    if lingering:
        brew_craft(
            "lingering_potion",
            from_potion,
            ingredient,
            "lingering_potion",
            to_potion,
            min_level=max(min_level, POTION_LINGERING_MIN_LEVEL),
        )

    split = to_potion.split(":", 1)
    base = split[0]
    root = split[1]

    if strong:
        brew(
            to_potion,
            "minecraft:glowstone_dust",
            f"{base}:strong_{root}",
            splash=splash,
            lingering=lingering,
            min_level=max(min_level, POTION_STRONG_MIN_LEVEL),
        )
    if long:
        brew(
            to_potion,
            "minecraft:redstone",
            f"{base}:long_{root}",
            splash=splash,
            lingering=lingering,
            min_level=max(min_level, POTION_LONG_MIN_LEVEL),
        )


brew(m("water"), m("nether_wart"), m("awkward"))
brew(m("awkward"), m("sugar"), m("swiftness"), True, True)
brew(m("awkward"), m("rabbit_foot"), m("leaping"), True, True)
brew(m("swiftness"), m("fermented_spider_eye"), m("slowness"), True, True, min_level=2)
brew(
    m("long_swiftness"),
    m("fermented_spider_eye"),
    m("long_slowness"),
    min_level=POTION_LONG_MIN_LEVEL,
)
brew(
    m("long_leaping"),
    m("fermented_spider_eye"),
    m("long_slowness"),
    min_level=POTION_LONG_MIN_LEVEL,
)
brew(m("leaping"), m("fermented_spider_eye"), m("slowness"), min_level=2)
brew(m("awkward"), m("blaze_powder"), m("strength"), True, True)
brew(m("awkward"), m("glistering_melon_slice"), m("healing"), True, False)
brew(m("healing"), m("fermented_spider_eye"), m("harming"), True, False, min_level=2)
brew(
    m("strong_healing"),
    m("fermented_spider_eye"),
    m("strong_harming"),
    min_level=POTION_STRONG_MIN_LEVEL,
)
brew(m("awkward"), m("spider_eye"), m("poison"), True, True)
brew(m("poison"), m("fermented_spider_eye"), m("harming"), min_level=2)
brew(
    m("strong_poison"),
    m("fermented_spider_eye"),
    m("strong_harming"),
    min_level=POTION_STRONG_MIN_LEVEL,
)
brew(m("awkward"), m("ghast_tear"), m("regeneration"), True, True)
brew(m("awkward"), m("magma_cream"), m("fire_resistance"), False, True)
brew(m("awkward"), m("pufferfish"), m("water_breathing"), False, True)
brew(m("awkward"), m("golden_carrot"), m("night_vision"), False, True)
brew(
    m("night_vision"),
    m("fermented_spider_eye"),
    m("invisibility"),
    False,
    True,
    min_level=2,
)
brew(
    m("long_night_vision"),
    m("fermented_spider_eye"),
    m("long_invisibility"),
    min_level=POTION_LONG_MIN_LEVEL,
)
brew(m("awkward"), m("turtle_shell"), m("turtle_master"), True, True, min_level=4)
brew(m("awkward"), m("phantom_membrane"), m("slow_falling"), False, True, min_level=5)
brew(m("water"), m("fermented_spider_eye"), m("weakness"), False, True)


# Yung's Caves
brew(
    m("awkward"),
    "yungscavebiomes:frost_lily",
    "yungscavebiomes:frost",
    False,
    False,
    min_level=4,
)
brew(
    "yungscavebiomes:frost",
    m("glowstone_dust"),
    "yungscavebiomes:strong_frost",
    False,
    False,
    min_level=max(POTION_STRONG_MIN_LEVEL, 4),
)
brew(
    m("fire_resistance"),
    m("fermented_spider_eye"),
    "yungscavebiomes:frost",
    False,
    False,
    min_level=4,
)
brew(
    m("strong_fire_resistance"),
    m("fermented_spider_eye"),
    "yungscavebiomes:strong_frost",
    False,
    False,
    min_level=4,
)


for potion in POTION_NAMES:
    brew_craft(
        "potion",
        potion,
        m("gunpowder"),
        "splash_potion",
        potion,
        min_level=POTION_SPLASH_MIN_LEVEL,
    )
    brew_craft(
        "splash_potion",
        potion,
        m("dragon_breath"),
        "lingering_potion",
        potion,
        min_level=POTION_LINGERING_MIN_LEVEL,
    )

    dp.alchemist.craft(
        [Item(m("arrow"), 8), Item(m("lingering_potion"), nbt={"Potion": potion})],
        Item(m("tipped_arrow"), 8, {"Potion": potion}),
        min_building_level=5,
    )


dp.save()
