#!/bin/bash

RETURN_CODE=0

echo "test file deletion"
FILE_TO_DELETE=$(mktemp)
echo "created $FILE_TO_DELETE"
sinkdir delete "$FILE_TO_DELETE"
if [ -e "$FILE_TO_DELETE" ]; then
echo "did not delete $FILE_TO_DELETE!"
RETURN_CODE=1
fi
echo "deleted $FILE_TO_DELETE"

echo "test file copy"
SOURCE_DIRECTORY=$(mktemp -d)
TARGET_DIRECTORY=$(mktemp -d)
touch "$SOURCE_DIRECTORY"/test.txt
sinkdir copy "$SOURCE_DIRECTORY" "$TARGET_DIRECTORY"
if [ ! -e "$TARGET_DIRECTORY"/test.txt ]; then
echo "File copy failed!"
RETURN_CODE=1
fi
echo "successfully copied test.txt from '$SOURCE_DIRECTORY' '$TARGET_DIRECTORY'"
rm -r "$SOURCE_DIRECTORY" "$TARGET_DIRECTORY"

exit $RETURN_CODE
