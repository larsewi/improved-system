# TODO

## Investigate what happens if table configs don't match

## There should be a way to not report records that are too big

## There should be a way to filter records. E.g. they may contain sensitive data
Find out how to do this without ruining the delta merging rules

## Add FFI log callback for C consumers
Expose a function to register a log callback so C consumers can capture
leech2 log messages in their own logging system instead of relying on stderr.

## Investigate how to package leech2

## Add validation function for block and patch
When we read something from disk, we don't know if it has been corrupted.

## When creating a new block, it should be opened with the EXECL flag
To protect agains overwriting existing blocks
