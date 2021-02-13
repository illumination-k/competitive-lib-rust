#!/usr/bin/python3

import argparse
import os
import tempfile
import subprocess
import sys

def get_module_name(line: str) -> str:
    """
    >>> get_module_name("use competitive::prime;")
    'prime'
    >>> get_module_name("use competitive::data_structures::union_find;")
    'data_structures/union_find'
    >>> get_module_name("use competitive::prime::*;")
    'prime'
    """
    line = line.rstrip(";\n")
    line = line.rstrip("::*")
    names = line.split(" ")[1].split("::")[1:]
    assert(len(names) != 0)
    if len(names) == 1:
        return names[0]
    
    assert(len(names) == 2)
    return os.path.join(names[0], names[1])

# settings
indent = " " * 4
cur_dir = os.path.dirname(os.path.abspath(__file__))


def main():
    parser = argparse.ArgumentParser("expand use::crate_name::xxxx")
    parser.add_argument("--bin", type=str)
    parser.add_argument("--crate_name", type=str, default="competitive")
    parser.add_argument("--stdout", action='store_true', help="stdout instead of overwrite")
    parser.add_argument("--test", action='store_true', help="test mode, path = test/bin.rs")
    args = parser.parse_args()

    module_names = []

    codes = []

    if args.test:
        path = "test/bin.rs"
        args.stdout = True
    else:
        path = f'./src/bin/{args.bin}.rs'
        outpath = path

    with open(path) as f:
        for line in f:
            if line.startswith("use"):
                if line.startswith(f'use {args.crate_name}::'):
                    line = line.replace(args.crate_name, args.crate_name + "_internal_mod")
                    module_names.append(get_module_name(line))

            codes.append(line.rstrip("\n"))

    all_modules_codes = ['', f'mod {args.crate_name}_internal_mod ' + '{']
    for module_name in module_names:
        module_path = os.path.join(cur_dir, "src", module_name + ".rs")
        depth = len(module_name.split(os.path.sep))
        
        module_codes = []
        if depth == 1:
            module_codes.append(indent + "pub mod " + module_name + " {")
        else:
            module_codes.append(indent + "pub mod " + os.path.split(module_name)[0] + " {")
            module_codes.append(indent*depth + "pub mod " + os.path.split(module_name)[1] + " {")
        
        # open module
        with open(module_path) as f:
            indent_num = (depth+1) * indent
            for line in f:
                # emit test mod
                if line.startswith("#[cfg(test)]"):
                    break
                
                # remove cargo snippet dependencies
                if "snippet" in line:
                    continue

                module_codes.append(indent_num + line.rstrip("\n"))    

        for i in range(depth, 0, -1):
            module_codes.append(indent * i + "}")

        all_modules_codes.extend(module_codes)
    all_modules_codes.append("}")
    
    codes.extend(all_modules_codes)

    # rust fmt
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_file = temp_dir + "output.rs"
        with open(temp_file, 'w') as w:
            w.write("\n".join(codes))
        subprocess.run(["rustfmt", temp_file], check=True)

        with open(temp_file, 'r') as f:
            lines = f.readlines()
            if args.stdout:
                print("".join(lines))
                sys.exit(0)
            with open(outpath, 'w') as w:
                w.write("".join(lines))


if __name__ == "__main__":
    main()