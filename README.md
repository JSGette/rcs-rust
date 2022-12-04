# Scalpel
It is a simple tool to easily and safely delete entries from Remote Cache.

## Design Document
bazel's RC contains of 2 components: action cache and content addressable storage.
Roughly speaking, first one contains list of output files and directories
of each action, second - contents of files and directories mentioned in the first one.
Another important requirement is that all CAS entries that're referred within AC entries
should exist BEFORE the entry can be added to AC. Meaning that if someone wants to 
do housekeeping of the cache should either wipe out the entire cache or somehow
link AC entries with CAS to safely delete from both.

This tool solves this problem. The plan is:
1. Decompile AC entries and parse list of blobs referred in CAS.
2. Filter AC entries according to some condition (example, entries older than N days).
3. Find all blobs in CAS mentioned in filtered AC entries.
4. If CAS blob isn't mentioned by any other AC entry that isn't filtered delete CAS.
5. After all CAS blobs mentioned in AC entry are safely deleted remove AC entry.

