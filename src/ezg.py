#!/usr/bin/env python3

# Create a long string in the format of
# <inputted file name>
# ```
# <inputted file contents>
# ```
#
# ...

# For every file in the list of files
# Then use the macOS clipboard command to copy the long string to the clipboard

import os
import sys

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 create_long_string.py <file1> <file2> ...")
        sys.exit(1)

    files = sys.argv[1:]

    full_string = ""

    for file in files:
        with open(file, 'r') as f:
            contents = f.read()
            full_string += f"{file}\n```\n{contents}\n```\n\n"

    os.system(f"echo '{full_string}' | pbcopy")

if __name__ == "__main__":
    main()
