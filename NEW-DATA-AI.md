# AQW Damage Formula Discrepancy Notes

Date: 2026-05-10

This document records ongoing analysis of the current AQWDex skill damage formula against in-game observations.

Important constraints from the user:

1. Do **not** assume public web information is useful for AQW damage math.
2. Assume provided base data is correct unless directly contradicted by saved build state:
   - Primary stats are correct.
   - Secondary stats are correct.
   - Weapon DPS is correct.
   - Weapon range is correct.
   - Static weapon behavior is correct.
   - Skill data is correct.
3. There are no weapon-roll differences in the data below. The weapon is static and consistently produces the same in-game values.
4. The discrepancy is therefore likely caused by an additional calculation step, rounding step, coercion step, or boost-ordering step that AQWDex currently does not model.
5. No existing `.rs` files were modified for this investigation.
6. No `backend-new` workspace was created because this note only updates formula research and does not provide a replacement implementation.

---

## 1. Current build assumptions

Build data location:

`~/.local/share/com.aqwdex.dev/builds.json`

The active build inspected was `Archpaladin`.

Relevant stats from the observed setup:

| Field | Value |
| --- | ---: |
| Weapon DPS | `85.0` |
| Weapon range | `1.0` |
| Attack Power | `877.0` |
| Spell Power | `877.0` |
| All Out | `120.0%` |
| Phy Out | `100.0%` |
| Mag Out | `140.12698%` |
| Enemy All In | `100.0%` |
| Enemy Phy In | `100.0%` |
| Enemy Mag In | `100.0%` |

Relevant skill setup:

| Skill | Damage | Source | Damage type |
| --- | ---: | --- | --- |
| Skill 1 | `1.1` | `AP2` | `Physical` |
| Skill 2 | `1.5` | `AP2` | `Physical` |
| Skill 4 | `1.35` | `APSP2` | `Physical` |
| Skill 5 | `5.5` | `SP2` | `Magical` |

Skill 5 has now been updated to `Magical`. The earlier note that Skill 5 was saved as `Physical` is obsolete.

---

## 2. Current backend formula reconstructed

Current formula from `backend/src/damage.rs`:

`damage = damage_source * outgoing_type_modifier * weapon_boost * skill_damage`

Relevant source values under current formula:

| Source | Formula | Current value |
| --- | --- | ---: |
| `AP1` | `weapon_dps + 0.1 * attack_power` | `85 + 87.7 = 172.7` |
| `AP2` | `2 * AP1 * weapon_range` | `345.4` |
| `SP1` | `weapon_dps + 0.1 * spell_power` | `85 + 87.7 = 172.7` |
| `SP2` | `2 * SP1 * weapon_range` | `345.4` |
| `APSP1` | `weapon_dps + 0.1 * AP + 0.1 * SP` | `260.4` |
| `APSP2` | `2 * APSP1 * weapon_range` | `520.8` |

Outgoing modifiers:

| Type | Formula | Current value |
| --- | --- | ---: |
| Physical | `All Out * Phy Out` | `1.2 * 1.0 = 1.2` |
| Magical | `All Out * Mag Out` | `1.2 * 1.4012698 = 1.68152376` |

Weapon boost values discussed:

| Scenario | Multiplier |
| --- | ---: |
| No weapon boost | `1.0` |
| `Boost51x50` | `1.51 * 1.5 = 2.265` |

---

## 3. Observed datasets

### 3.1 No weapon boost

User-provided no-boost in-game values:

| Skill | In-game | Calculator | Difference | Ratio |
| --- | ---: | ---: | ---: | ---: |
| Skill 1 | `457` | `455.9280` | `+1.0720` | `1.0023512` |
| Skill 2 | `623` | `621.7200` | `+1.2800` | `1.0020588` |
| Skill 4 | `845` | `843.6960` | `+1.3040` | `1.0015456` |
| Skill 5 | `3200` | `3194.3907` | `+5.6093` | `1.0017560` |

These calculator values are exactly the old boosted calculator values divided by `2.265`, which matches the current formula's structure.

### 3.2 With `Boost51x50`

Previously provided boosted in-game values:

| Skill | In-game | Calculator | Difference | Ratio |
| --- | ---: | ---: | ---: | ---: |
| Skill 1 | `1036` | `1032.6769` | `+3.3231` | `1.0032179` |
| Skill 2 | `1411` | `1408.1958` | `+2.8042` | `1.0019913` |
| Skill 4 | `1914` | `1910.9714` | `+3.0286` | `1.0015848` |
| Skill 5 | `7249` | `7235.2949` | `+13.7051` | `1.0018942` |

---

## 4. Immediate conclusions from no-boost data

The no-boost dataset is especially useful because it removes item boost stacking from the equation.

Because the static weapon and all provided stats are assumed correct, the missing step must occur somewhere after or during the current base damage pipeline.

The current raw no-boost values are:

| Skill | Current raw no-boost | In-game integer |
| --- | ---: | ---: |
| Skill 1 | `455.9280` | `457` |
| Skill 2 | `621.7200` | `623` |
| Skill 4 | `843.6960` | `845` |
| Skill 5 | `3194.3907` | `3200` |

Final rounding alone does not explain this. Rounding current raw values gives:

| Skill | Current raw | Rounded |
| --- | ---: | ---: |
| Skill 1 | `455.9280` | `456` |
| Skill 2 | `621.7200` | `622` |
| Skill 4 | `843.6960` | `844` |
| Skill 5 | `3194.3907` | `3194` |

So there is a small missing positive step before display rounding.

---

## 5. Python-tested candidate: a tiny final scalar before display

For the no-boost dataset only, a common hidden scalar applied after the current raw damage but before final rounding can explain all four values.

Assuming final display uses nearest-integer rounding, the scalar `C` must satisfy:

| Skill | Valid `C` interval for no-boost rounding |
| --- | ---: |
| Skill 1 | `1.0012546` to `1.0034479` |
| Skill 2 | `1.0012546` to `1.0028630` |
| Skill 4 | `1.0009529` to `1.0021382` |
| Skill 5 | `1.0015995` to `1.0019125` |

Intersection:

`1.0015995 <= C < 1.0019125`

So a hidden scalar around:

`C ≈ 1.00175` to `1.00185`

would make all no-boost values display correctly.

Example with `C = 1.0018`:

| Skill | Current raw | Raw after `* 1.0018` | Rounded | In-game |
| --- | ---: | ---: | ---: | ---: |
| Skill 1 | `455.9280` | `456.7487` | `457` | `457` |
| Skill 2 | `621.7200` | `622.8391` | `623` | `623` |
| Skill 4 | `843.6960` | `845.2147` | `845` | `845` |
| Skill 5 | `3194.3907` | `3200.1406` | `3200` | `3200` |

This is the cleanest single-step explanation for the no-boost dataset.

However, this scalar alone does not fully explain the boosted dataset unless weapon boost application also has its own rounding/order behavior.

---

## 6. Python-tested candidate: additive source offset

Another equivalent explanation for the no-boost dataset is a tiny additive offset to the damage source before outgoing modifiers and skill multipliers.

Current sources:

| Source | Current |
| --- | ---: |
| `AP2` / `SP2` | `345.4` |
| `APSP2` | `520.8` |

If a flat offset of roughly `+0.60` to `+0.65` is added to the source value before type modifiers and skill multipliers, all no-boost observations round correctly.

Example with `source += 0.60`:

| Skill | Raw after source offset | Rounded | In-game |
| --- | ---: | ---: | ---: |
| Skill 1 | `456.7200` | `457` | `457` |
| Skill 2 | `622.8000` | `623` | `623` |
| Skill 4 | `844.6680` | `845` | `845` |
| Skill 5 | `3199.9397` | `3200` | `3200` |

Example with `source += 0.65`:

| Skill | Raw after source offset | Rounded | In-game |
| --- | ---: | ---: | ---: |
| Skill 1 | `456.7860` | `457` | `457` |
| Skill 2 | `622.8900` | `623` | `623` |
| Skill 4 | `844.7490` | `845` | `845` |
| Skill 5 | `3200.4021` | `3200` | `3200` |

This candidate is mathematically plausible, but with the current data it is difficult to distinguish from the small hidden scalar. Both produce nearly the same displayed integers.

To distinguish these two possibilities, we need tests where AP/SP or weapon DPS changes while outgoing modifiers stay the same.

---

## 7. Effective source values implied by no-boost observations

If the observed integer values are treated as exact raw targets, the implied effective source values are:

| Skill | Source type | Current source | Implied source from in-game integer |
| --- | --- | ---: | ---: |
| Skill 1 | `AP2` | `345.4` | `346.2121` |
| Skill 2 | `AP2` | `345.4` | `346.1111` |
| Skill 4 | `APSP2` | `520.8` | `521.6049` |
| Skill 5 | `SP2` | `345.4` | `346.0065` |

These suggest the current source term is low by roughly `+0.6` to `+0.8`, but because the observed values are integer display values, these should be interpreted as approximate intervals rather than exact true raw values.

---

## 8. Boosted dataset requires an additional boost-ordering / rounding step

The no-boost discrepancy is smaller and can be explained by either:

1. A tiny final scalar around `1.0017–1.0019`, or
2. A small source offset around `+0.60–0.65`.

The boosted dataset adds another clue: AQW likely does not apply item boosts as one pure floating-point multiplication followed by final display rounding.

The current calculator uses:

`boosted_damage = raw_damage * 1.51 * 1.5`

But in-game boost behavior may apply rounding or integer coercion between boost layers.

A useful test pattern found in Python:

`floor(ceil(round(no_boost_damage) * 1.51) * 1.5)`

This reproduces three boosted values if the no-boost display values are used as the base:

| Skill | No-boost integer | `floor(ceil(round(no_boost) * 1.51) * 1.5)` | Boosted in-game |
| --- | ---: | ---: | ---: |
| Skill 1 | `457` | `1036` | `1036` |
| Skill 2 | `623` | `1411` | `1411` |
| Skill 4 | `845` | `1914` | `1914` |
| Skill 5 | `3200` | `7248` | `7249` |

Skill 5 is off by one in this model because `3200 * 1.51 = 4832` exactly, so the `ceil` step does not increase it. If the hidden true no-boost Skill 5 raw value is slightly above `3200.0` while still displaying as `3200`, then this same boost-ordering model produces `7249`:

- If raw Skill 5 is approximately `3200.22` to `< 3200.5`, it displays as `3200` with nearest rounding.
- `ceil(raw * 1.51)` becomes `4833`.
- `floor(4833 * 1.5) = 7249`.

This means the boosted data strongly suggests boost stacking has an integer/ceiling step somewhere, especially around the `1.51` multiplier.

---

## 9. Combined constraints from no-boost and boosted values

Assuming nearest-integer display rounding both before and after the boosted calculation, the hidden true no-boost raw values must fall in these intervals:

| Skill | Current raw no-boost | Required hidden raw interval | Minimum missing raw |
| --- | ---: | ---: | ---: |
| Skill 1 | `455.9280` | `[457.1744, 457.5000)` | `+1.2464` |
| Skill 2 | `621.7200` | `[622.7373, 623.1788)` | `+1.0173` |
| Skill 4 | `843.6960` | `[844.8124, 845.2539)` | `+1.1164` |
| Skill 5 | `3194.3907` | `[3200.2208, 3200.5000)` | `+5.8301` |

The boosted Skill 1 value is the hardest to satisfy with a single global scalar or source offset. Skill 1 may have an auto-attack-specific ordering step, or the boost path may use the displayed integer damage as an intermediate for some skills.

---

## 10. What is now considered unlikely

### 10.1 Weapon roll differences

Rejected by user data. The weapon is static and consistently produces the same values.

### 10.2 Incorrect weapon DPS / weapon range

Do not assume this. The user has stated weapon DPS, range, and static weapon data are correct.

### 10.3 Incorrect primary/secondary stats

Do not assume this. The user has stated base stat data is correct.

### 10.4 Enemy modifiers

Current enemy incoming stats are neutral. Enemy modifiers do not explain this dataset.

---

## 11. Current best working model

The most plausible model from current data is a two-part missing process:

### Part A: hidden pre-display damage adjustment

Before final display, AQW appears to apply a very small positive adjustment to raw skill damage.

Two equivalent forms fit the no-boost dataset:

1. Hidden final scalar: `raw *= approximately 1.0017–1.0019`
2. Hidden source offset: `damage_source += approximately 0.60–0.65`

The no-boost dataset alone cannot distinguish these.

### Part B: boost stacking has integer/ceil/floor behavior

The boosted data indicates that `Boost51x50` is likely not just one smooth float multiplier of `2.265` applied at the end.

A promising experimental boost ordering is:

`boosted = floor(ceil(base_damage * 1.51) * 1.5)`

Possibly with `base_damage` being either:

- an already rounded display-ish value, or
- the hidden adjusted raw damage from Part A.

This explains why boosted discrepancies are slightly different from no-boost discrepancies.

---

## 12. Recommended next tests

No code changes should be made yet. The next data should isolate the missing step.

### Test A: 51% boost only

Record the same four skills with only a `51%` weapon boost.

This tests whether the suspected `ceil(base * 1.51)` step exists.

### Test B: 50% boost only

Record the same four skills with only a `50%` boost.

This tests whether the `1.5` step floors, rounds, or carries float values.

### Test C: no All Out passive

Record the same four no-boost skills with All Out at `100%` instead of `120%`.

If the missing step is a source offset, the absolute difference should scale with the outgoing modifier differently than if the missing step is a final scalar.

### Test D: different AP/SP values, same weapon

Change AP/SP significantly while keeping the same static weapon and same outgoing modifiers.

If the missing step is a source offset, the inferred `+source` correction should remain close to constant.

If it is a final scalar, the ratio should remain close to constant.

### Test E: Skill 1 separately

Skill 1 behaves slightly differently in the boosted dataset. Because Skill 1 is often an auto attack, it may have a different boost/rounding path.

Record Skill 1 with:

- no boost
- 51% only
- 50% only
- 51% + 50%

This should clarify whether Skill 1 uses display integer damage as an intermediate.

---

## 13. Recommendation before code changes

Do not change the production backend formula yet.

The current formula structure is very close. The strongest current evidence is not that AP/SP/source/outgoing/skill scalar math is wrong, but that AQW has one or more hidden post-processing steps:

1. A tiny hidden damage adjustment before display, and/or
2. Integer coercion during boost stacking.

If an experimental implementation is created later, it should go in a new `backend-new` workspace and should expose the rounding/boost mode as an experiment rather than replacing the current formula outright.
