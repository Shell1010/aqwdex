import math


class SomeEnhObject:
    def __init__(self):
        self.enh_level = 100
        self.enh_rarity = 6
        self.PatternID = "28"

GST_BASE = 12
STATS_EXPONENT = 1.33
GST_GOAL = 572
MAX_LEVEL = 100
STATS = ["STR","END","DEX","INT","WIS","LCK"]
ENH_PATTERN_TREE = {
    "1": {
         "ID": 1,
         "sName": "Adventurer",
         "sDesc": "none",
         "iSTR": 16,
         "iDEX": 16,
         "iEND": 18,
         "iINT": 16,
         "iWIS": 16,
         "iLCK": 0
    },
    "2": {
         "ID": 2,
         "sName": "Fighter",
         "sDesc": "M1",
         "iSTR": 44,
         "iDEX": 13,
         "iEND": 43,
         "iINT": 0,
         "iWIS": 0,
         "iLCK": 0
    },
    "3": {
         "ID": 3,
         "sName": "Thief",
         "sDesc": "M2",
         "iSTR": 30,
         "iDEX": 45,
         "iEND": 25,
         "iINT": 0,
         "iWIS": 0,
         "iLCK": 0
    },
    "4": {
         "ID": 4,
         "sName": "Armsman",
         "sDesc": "M4",
         "iSTR": 38,
         "iDEX": 36,
         "iEND": 26,
         "iINT": 0,
         "iWIS": 0,
         "iLCK": 0
    },
    "5": {
         "ID": 5,
         "sName": "Hybrid",
         "sDesc": "M3",
         "iSTR": 28,
         "iDEX": 20,
         "iEND": 25,
         "iINT": 27,
         "iWIS": 0,
         "iLCK": 0
    },
    "6": {
         "ID": 6,
         "sName": "Wizard",
         "sDesc": "C1",
         "iSTR": 0,
         "iDEX": 0,
         "iEND": 10,
         "iINT": 50,
         "iWIS": 20,
         "iLCK": 20
    },
    "7": {
         "ID": 7,
         "sName": "Healer",
         "sDesc": "C2",
         "iSTR": 0,
         "iDEX": 0,
         "iEND": 40,
         "iINT": 45,
         "iWIS": 15,
         "iLCK": 0
    },
    "8": {
         "ID": 8,
         "sName": "Spellbreaker",
         "sDesc": "C3",
         "iSTR": 0,
         "iDEX": 0,
         "iEND": 20,
         "iINT": 40,
         "iWIS": 30,
         "iLCK": 10
    },
    "9": {
         "ID": 9,
         "sName": "Lucky",
         "sDesc": "S1",
         "iSTR": 10,
         "iDEX": 10,
         "iEND": 10,
         "iINT": 10,
         "iWIS": 10,
         "iLCK": 50
    },
    "10": {
         "ID": 10,
         "sName": "Forge",
         "sDesc": "Blacksmith",
         "iSTR": 25,
         "iDEX": 0,
         "iEND": 0,
         "iINT": 25,
         "iWIS": 0,
         "iLCK": 50
    },
    "14": {
         "ID": 25,
         "sName": "Vim",
         "sDesc": "SmithP2",
         "iSTR": 10,
         "iDEX": 130,
         "iEND": -90,
         "iINT": 0,
         "iWIS": 0,
         "iLCK": 50
    },
    "15": {
         "ID": 32,
         "sName": "Hearty",
         "sDesc": "Grimskull Troll Enhancement",
         "iSTR": -20,
         "iDEX": -20,
         "iEND": 150,
         "iINT": -20,
         "iWIS": -20,
         "iLCK": -20
    },
    "26": {
         "ID":26,
         "sName":"Examen",
         "sDesc":"SmithP2",
         "iSTR":0,
         "iDEX":0,
         "iEND":-90,
         "iINT":10,
         "iWIS":130,
         "iLCK":50,
    },
    "27": {
         "ID":27,
         "sName":"Pneuma",
         "sDesc":"SmithP2",
         "iSTR":24,
         "iDEX":24,
         "iEND":-90,
         "iINT":118,
         "iWIS":24,
         "iLCK":0,
    },
    "28": {
         "ID":28,
         "sName":"Anima",
         "sDesc":"SmithP2",
         "iSTR":134,
         "iDEX":24,
         "iEND":-90,
         "iINT":16,
         "iWIS":16,
         "iLCK":0,
    }
}


GEAR_RATIOS = {
    "he": 0.25,
    "ar": 0.25,
    "ba": 0.2,
    "Weapon": 0.33
}

def gstTotal(enh_level, enh_rarity):
    total = round(enh_level + enh_rarity - 1)

    # int in actionscript is basically math.floor
    return math.floor(round(12 + ((total) * 560) / (MAX_LEVEL - 1)))


def getStats(enhancement: SomeEnhObject, gear_slot):
    total = gstTotal(enhancement.enh_level, enhancement.enh_rarity)
    gear_stat_total = round( total * GEAR_RATIOS[gear_slot])
    print(gear_stat_total)
    some_temp_stat_obj = {}
    stat_val = ""
    other_count = 0
    some_stat_array = ["iEND","iSTR","iINT","iDEX","iWIS","iLCK"]
    some_other_thing = ""
    some_neg_value = -1
    returned_obj = {}

    if enhancement.PatternID != None:
        enh_pattern = ENH_PATTERN_TREE[enhancement.PatternID]
    # if enhancement.EnhPatternID != None:
    #     enh_pattern = ENH_PATTERN_TREE[enhancement.EnhPatternID]
    if enh_pattern != None:
        count = 0
        while count < len(STATS):
            stat_val = "i" + STATS[count]
            if enh_pattern[stat_val] != None:
                some_temp_stat_obj[stat_val] = round(gear_stat_total * enh_pattern[stat_val] / 100)
                print(gear_stat_total, some_temp_stat_obj[stat_val], enh_pattern[stat_val])
                other_count += some_temp_stat_obj[stat_val]
  
            count += 1
            # repeats for every single stat
            # till enh_pattern_obj is finished
    # theres more but its irrelevant - MAYBE NOT!!!?!?!??!?!?!

    index = 0 # loc9# adds 1 to the value
    while other_count < gear_stat_total:
        some_other_thing = some_stat_array[index] # stat name iEND, iSTR, iINT, etc
        # print(index)
        if some_temp_stat_obj[some_other_thing] != None: # if stat is not none and it exists
            some_temp_stat_obj[some_other_thing] += 1
            # print(some_temp_stat_obj[some_other_thing])
            other_count += 1
        index += 1
        # ok but wh
            
        if index > len(some_stat_array) - 1:
            index = 0 # resets if index reaches end of array
        # the purpose of this loop is to add additional stats
        # if after rounding with enhancement coeffs you haven't reached the true gst
        # E.g Rounded value is 136 and GST is 138
        # it begins this loop
        # it adds +1 to each stat, in this priority
        # ["STR","END","DEX","INT","WIS","LCK"]
        # This is why sometimes you have extra +1 for stats
        # AQW is WEIRD

        

    count = 0
    while count < len(STATS):
        some_neg_value = some_temp_stat_obj["i" + STATS[count]]

        if count != None and count != "0":
            returned_obj["$" + STATS[count]] = some_neg_value

        count += 1

        # I also don't understand why this loop is heregetStatsA
        # Basically just adds $ to each stat, returns new object
    
    return returned_obj

def main():
    enh = SomeEnhObject()
    print(getStats(enh, "he"))

    
main()