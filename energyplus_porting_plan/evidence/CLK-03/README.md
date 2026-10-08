# CLK-03 retained review evidence

The [closure evidence](../CLK-03.json) records the completed bounded card at
implementation commit `df88eb4c7b03cd8a04ad09d5fce3300a4d5b7b22`.
Unit comparisons are 239,178/0 mismatches; the new source-matched production
comparisons are 1,742,058/0. The earlier 1,647,956/1 failure remains failed.

[archive.json](archive.json) and its [supplement](archive-supplement.json) map
each selected local proof or source to its byte-identical Git copy. These 46
proof documents and 27 executed or reviewed source files occupy about 13.17 MB.
Original path strings, execution
times, source identities and proof contents are preserved. Large traces,
native binaries remain local under ignored `.runtime`.
This archive is a selected review packet, not a complete native build cache.
The local `.gitattributes` disables line-ending conversion for archived proof
and source bytes so their recorded hashes also match the Git objects.
The local `.ignore` excludes historical copies from default ripgrep searches;
use `rg --no-ignore` for an explicit evidence search.

The files under `sources/` retain their original directory assumptions and
must be read as historical source records. They are not relocated executable
entrypoints. Existing canonical unit tools remain under `tools/porting/`.

The [actual retirement record](reports/retirement.json) records successful
single-file removal of the two CLK-03 debug originals after actual card
closure and independent result review. It reclaimed 2,593,250,328 bytes.
Both validated executable derivatives, the failed original/derivative pair,
the CLK-02 derivative and fourteen earlier products/libraries remain intact.
The [version 3 registry](reports/retired-registry.json) preserves the historical
identities and removal receipts. A hash or PE map cannot recreate discarded
debug bytes; historical launchers requiring those live originals will reject
a new launch after retirement.

The initial archive attempt stopped at its document-size cap after two small
copies. Its actual failed metadata receipt is retained in `archive.json`.
The corrected archive resumed only identical files; a separate supplement
retains the three larger reports below 4 MB without changing their contents
or the first manifest. No engine, numerical comparison, or card gate was rerun or
changed while creating this archive.
