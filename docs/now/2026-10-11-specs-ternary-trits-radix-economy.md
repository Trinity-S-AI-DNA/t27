# NOW -- specs/ternary: trits, radix_economy -- the logic of the course "The ternary machine" (2026-10-11)

## specs/ternary: trits + radix_economy (Closes #8978)

- trits: balanced digits by the offset 111...1 (unsigned, no signed division), negation, rounding by cutting, Kleene gates, half/full/ripple adders, multiplying without a multiplier, five trits in a byte, three-way compare, a ternary activation; 18 tests, 4108 asserts
- radix_economy: digits per base, cost b * digits, b / ln b; a million costs 39 in base 3, 40 in bases 2 and 4
- 8 of 8 negative controls caught
