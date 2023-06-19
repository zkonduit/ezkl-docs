# Run with something like:
# python extract.py '../ezkl' '../ezkl-docs/Command_Line_Interface.md' | bash


import sys
#print(sys.argv)

ezkl_path = sys.argv[1] #"../ezkl"
doc_path = sys.argv[2] #'../ezkl-docs/Command_Line_Interface.md'

with open(doc_path, 'r') as f:
    in_code_block = False
    for line in f:
        stripped_line = line.strip()
        if stripped_line.startswith('```'):
            if ("ignore" not in stripped_line and stripped_line.startswith('```bash')) or in_code_block: 
                in_code_block = not in_code_block
        elif in_code_block:
            stripped_line = stripped_line.replace("~/ezkl",ezkl_path)
            print(stripped_line)
