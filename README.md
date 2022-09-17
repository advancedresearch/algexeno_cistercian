# Algexeno-Cistercian
Visual Algexenotation with Cistercian numerals as hyperprimes

![stroke](https://user-images.githubusercontent.com/1743862/190861736-22775460-2118-460e-845c-41df99290ddb.gif)

This animation shows how to write "2022" with this visual representation of Algexenotation.

### What is Algexenotation?

Algexenotation is a way to represent multisets as natural numbers with algebraic compression.

For example:

```text
2022' = 0*1*(1+0^0*(2+0^0))
```

For more information, see the [Algexenotation](https://github.com/advancedresearch/algexenotation) project.

### What are Cistercian numerals?

[Cistercian numberals](https://en.wikipedia.org/wiki/Cistercian_numerals) were developed by the Cistercian monastic order in in the early thirteenth century at about the time that Arabic numerals were introduced to northwestern Europe. They are more compact than Arabic or Roman numerals, with a single glyph able to indicate any integer from 1 to 9,999.

![Cistercian numberals](https://upload.wikimedia.org/wikipedia/commons/thumb/6/67/Cistercian_digits_%28vertical%29.svg/1920px-Cistercian_digits_%28vertical%29.svg.png)

### Motivation

Develop a readable representation of Algexenotation that does not overlap with mainstream usage of decimals.

- Cover 64 bit range
- Making powers easier to read
- Making sums easier to read

### Design

There are so few hyperprimes in the 64 bit range that using the full range of Cistercian numerals is not needed.
This property is exploited to better represent powers as single Cistercian numerals, plus adding support for zero using a horizonal line separator.

- Original form: True Cistercian numerals
- Evaluated form: Modified Cistercian numerals to cover hyperprimes in 64 bit range

The modified Cistercian numerals counts `1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 30, 40, 50, 60, ...`.
