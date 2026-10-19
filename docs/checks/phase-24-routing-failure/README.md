# Preserved diagnostic evidence

These files retain superseded diagnostic failures, not accepted phase evidence.
The first gate incorrectly passed maintenance phase 24 to the original native
implementation checker (whose final milestone is 8). Commit bab1260 fixes the
routing and adds a regression assertion. The original failed gate and its private
scan receipt remain unchanged here.

`private-scan-concurrent.json` captured a private scan while gate bookkeeping was
changing. It found zero credential or package-value matches, but correctly failed
`sourceUnchanged`; it does not supersede `../phase-24-private-material.json`.
A subsequent scan on the stable candidate passed before qualification resumed.
