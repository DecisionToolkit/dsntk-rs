# Algorithm of recognizing a decision table

## STEP 1

Take a decision table defined as Unicode text, for example:

```text
 ┌───────────────────────────┐
 │   information item name   │
 ├───┬────────────────────┬──┴─────────────────╥────────────────────┐
 │ U │ input expression 1 │ input expression 2 ║    output label    │
 │   ├────────────────────┼────────────────────╫────────────────────┤
 │   │ input value 1a,    │ input value 2a,    ║ output value 1a,   │
 │   │   input value 1b   │   input value 2b   ║   output value 1b  │
 ╞═══╪════════════════════╪════════════════════╬════════════════════╡
 │ 1 │                    │  input entry 2.1   ║  output entry 1.1  │
 ├───┤  input entry 1.1   ├────────────────────╫────────────────────┤
 │ 2 │                    │  input entry 2.2   ║  output entry 1.2  │
 ├───┼────────────────────┼────────────────────╫────────────────────┤
 │ 3 │  input entry 1.2   │         -          ║  output entry 1.3  │
 ├───┼────────────────────┼────────────────────╫────────────────────┤
 │ 4 │  input entry 1.3   │  input entry 2.3   ║  output entry 1.4  │
 └───┴────────────────────┴────────────────────╨────────────────────┘
```

## STEP 2

Remove all characters except box drawing characters.

```text
 ┌───────────────────────────┐
 │                           │
 ├───┬────────────────────┬──┴─────────────────╥────────────────────┐
 │   │                    │                    ║                    │
 │   ├────────────────────┼────────────────────╫────────────────────┤
 │   │                    │                    ║                    │
 │   │                    │                    ║                    │
 ╞═══╪════════════════════╪════════════════════╬════════════════════╡
 │   │                    │                    ║                    │
 ├───┤                    ├────────────────────╫────────────────────┤
 │   │                    │                    ║                    │
 ├───┼────────────────────┼────────────────────╫────────────────────┤
 │   │                    │                    ║                    │
 ├───┼────────────────────┼────────────────────╫────────────────────┤
 │   │                    │                    ║                    │
 └───┴────────────────────┴────────────────────╨────────────────────┘
```

## STEP 3

Remove the information item cell if present.

```text
 ┌───┬────────────────────┬────────────────────╥────────────────────┐
 │   │                    │                    ║                    │
 │   ├────────────────────┼────────────────────╫────────────────────┤
 │   │                    │                    ║                    │
 │   │                    │                    ║                    │
 ╞═══╪════════════════════╪════════════════════╬════════════════════╡
 │   │                    │                    ║                    │
 ├───┤                    ├────────────────────╫────────────────────┤
 │   │                    │                    ║                    │
 ├───┼────────────────────┼────────────────────╫────────────────────┤
 │   │                    │                    ║                    │
 ├───┼────────────────────┼────────────────────╫────────────────────┤
 │   │                    │                    ║                    │
 └───┴────────────────────┴────────────────────╨────────────────────┘
```

## STEP 4

Replace all double lines with single lines.

```text
 ┌───┬────────────────────┬────────────────────┬────────────────────┐
 │   │                    │                    │                    │
 │   ├────────────────────┼────────────────────┼────────────────────┤
 │   │                    │                    │                    │
 │   │                    │                    │                    │
 ├───┼────────────────────┼────────────────────┼────────────────────┤
 │   │                    │                    │                    │
 ├───┤                    ├────────────────────┼────────────────────┤
 │   │                    │                    │                    │
 ├───┼────────────────────┼────────────────────┼────────────────────┤
 │   │                    │                    │                    │
 ├───┼────────────────────┼────────────────────┼────────────────────┤
 │   │                    │                    │                    │
 └───┴────────────────────┴────────────────────┴────────────────────┘
```

## STEP 5

Add lacking single line segments.
Map the full grid cells to regions from [STEP 2](#step-2).

```text
 ┌───┬────────────────────┬────────────────────┬────────────────────┐
 │   │                    │                    │                    │
 ├───┼────────────────────┼────────────────────┼────────────────────┤
 │   │                    │                    │                    │
 │   │                    │                    │                    │
 ├───┼────────────────────┼────────────────────┼────────────────────┤
 │   │                    │                    │                    │
 ├───┼────────────────────┼────────────────────┼────────────────────┤
 │   │                    │                    │                    │
 ├───┼────────────────────┼────────────────────┼────────────────────┤
 │   │                    │                    │                    │
 ├───┼────────────────────┼────────────────────┼────────────────────┤
 │   │                    │                    │                    │
 └───┴────────────────────┴────────────────────┴────────────────────┘
```

## STEP 6

Rearrange the matrix such a way, that the content cells from [STEP 5](#step-5)
are split with cells simulating double lines. Cells with double lines are marked with `╳`.

```text
 ┌───┬───┬───┬───┬───┐
 │   │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │   │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ ╳ │ ╳ │ ╳ │ ╳ │ ╳ │
 ├───┼───┼───┼───┼───┤
 │   │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │   │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │   │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │   │   │   │ ╳ │   │
 └───┴───┴───┴───┴───┘
```

## STEP 7.

Check if the top left cell points to the region with valid hit policy. If yes, then check if all the cells
in the first column above the first double line point to the same region with hit policy. Example with hit policy
placed in the top left corner is shown below. Hit policy is marked as H. Such table is a decision table with
rules as rows. First column below the first double line should contain rule numbers marked as N.

```text
 ┌───┬───┬───┬───┬───┐
 │ H │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ H │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ ╳ │ ╳ │ ╳ │ ╳ │ ╳ │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 └───┴───┴───┴───┴───┘
```

## STEP 7a.

If there is more than one row above the first double line, then the row near the double contains cells
with default values, marked as D. In such case all the cells above default values in each column should point
to the same region (on the left from the double line). These regions are input expressions marked as I.
To the right is the output label.

```text
 ┌───┬───┬───┬───┬───┐
 │ H │ I │ I │ ╳ │ O │
 ├───┼───┼───┼───┼───┤
 │ H │ D │ D │ ╳ │ D │
 ├───┼───┼───┼───┼───┤
 │ ╳ │ ╳ │ ╳ │ ╳ │ ╳ │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 └───┴───┴───┴───┴───┘
```

## STEP 7b.

No default values.

```text
 ┌───┬───┬───┬───┬───┐
 │ H │ I │ I │ ╳ │ O │
 ├───┼───┼───┼───┼───┤
 │ ╳ │ ╳ │ ╳ │ ╳ │ ╳ │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 ├───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │
 └───┴───┴───┴───┴───┘
```

## STEP 7c.

Multiple outputs.

```text
 ┌───┬───┬───┬───┬───┬───┐
 │ H │ I │ I │ ╳ │ O │ O │
 ├───┼───┼───┼───┼───┼───┤
 │ H │ I │ I │ ╳ │ C │ C │
 ├───┼───┼───┼───┼───┼───┤
 │ H │ D │ D │ ╳ │ D │ D │
 ├───┼───┼───┼───┼───┼───┤
 │ ╳ │ ╳ │ ╳ │ ╳ │ ╳ │ ╳ │
 ├───┼───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │   │
 ├───┼───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │   │
 ├───┼───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │   │
 ├───┼───┼───┼───┼───┼───┤
 │ N │   │   │ ╳ │   │   │
 └───┴───┴───┴───┴───┴───┘
```

## STEP 7d.

Rows below the double line on the left are input entries marked as P.
Rows below the first double line to the right are the output entries marked as T

```text
 ┌───┬───┬───┬───┬───┐
 │ H │ I │ I │ ╳ │ O │
 ├───┼───┼───┼───┼───┤
 │ H │ D │ D │ ╳ │ D │
 ├───┼───┼───┼───┼───┤
 │ ╳ │ ╳ │ ╳ │ ╳ │ ╳ │
 ├───┼───┼───┼───┼───┤
 │ N │ P │ P │ ╳ │ T │
 ├───┼───┼───┼───┼───┤
 │ N │ P │ P │ ╳ │ T │
 ├───┼───┼───┼───┼───┤
 │ N │ P │ P │ ╳ │ T │
 ├───┼───┼───┼───┼───┤
 │ N │ P │ P │ ╳ │ T │
 └───┴───┴───┴───┴───┘
```

# Algorithm of recognizing the preferred orientation of the decision table

Algorithm of recognizing the preferred orientation of the decision table is presented
below as a decision table. Correctly constructed decision table may be presented
horizontally (rules as rows), vertically (rules as columns) or as crosstab.

Every decision table must contain minimum one vertical and one horizontal double line.
If the decision table contains annotation clause, then the must be second horizontal
or vertical line present. When the decision table does not contain any double lines
or contains more than two horizontal or two vertical lines, then such decision table
is invalid and will not be processed. This situation is covered by rules **1** and **2**.

Rules **3** and **4** detect `horizontal decision table`. Horizontal decision table must
consist of single horizontal double line, one or two vertical double lines, hit policy
marker must be placed in the top left corner, and the rule numbers must be placed
in the first left column below the horizontal double lines. When all these conditions
are met, decision table is oriented _horizontally_ (rules as rows).

Rules **5** and **6** detect `vertical decision table`. Vertical decision table contains
single vertical double line and one or two horizontal double line. When vertical decision
table contains annotation clause, then contains two horizontal double lines.
In horizontal decision table the hit policy marker must be placed in the left bottom
corner, and the rule numbers must be placed in the last row on the right side of the vertical
double line. When all these conditions are met, decision table is oriented _vertically_
(rules as columns).

Rule **7** detects `crosstab decision table`. Crosstab decision table contains exactly one
vertical and one horizontal line. Crosstab decision table does not contain any hit policy
marker nor rule numbers. When all these conditions are met, decision table is a _crosstab_.

```text
 ┌─────────────────────────┐
 │  Preferred orientation  │
 ├───┬────────────────────┬┴─────────────────────┬──────────────┬──────────────╥─────────────┐
 │ F │ Number of vertical │ Number of horizontal │  Hit policy  │    Rule      ║  Preferred  │
 │   │    double lines    │     double lines     │              │   numbers    ║ orientation │
 │   ├────────────────────┼──────────────────────┼──────────────┼──────────────╫─────────────┤
 │   │    <1, 1, 2, >2    │     <1, 1, 2, >2     │  top-left,   │ left-below,  ║ horizontal, │
 │   │                    │                      │ bottom-left, │ right-after, ║  vertical,  │
 │   │                    │                      │  not-found   │  not-found   ║  crosstab,  │
 │   │                    │                      │              │              ║    error    │
 ╞═══╪════════════════════╪══════════════════════╪══════════════╪══════════════╬═════════════╡
 │ 1 │       <1, >2       │         -            │      -       │      -       ║    error    │
 ├───┼────────────────────┼──────────────────────┼──────────────┼──────────────╫─────────────┤
 │ 2 │         -          │       <1, >2         │      -       │      -       ║    error    │
 ├───┼────────────────────┼──────────────────────┼──────────────┼──────────────╫─────────────┤
 │ 3 │         2          │         1            │  top-left    │ left-below   ║ horizontal  │
 ├───┼────────────────────┼──────────────────────┼──────────────┼──────────────╫─────────────┤
 │ 4 │         1          │         1            │  top-left    │ left-below   ║ horizontal  │
 ├───┼────────────────────┼──────────────────────┼──────────────┼──────────────╫─────────────┤
 │ 5 │         1          │         2            │ bottom-left  │ right-after  ║  vertical   │
 ├───┼────────────────────┼──────────────────────┼──────────────┼──────────────╫─────────────┤
 │ 6 │         1          │         1            │ bottom-left  │ right-after  ║  vertical   │
 ├───┼────────────────────┼──────────────────────┼──────────────┼──────────────╫─────────────┤
 │ 7 │         1          │         1            │  not-found   │  not-found   ║  crosstab   │
 └───┴────────────────────┴──────────────────────┴──────────────┴──────────────╨─────────────┘
```