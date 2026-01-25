#!/usr/bin/env python3
"""
Generate api.rs from functions.txt

This script reads the WAAPI function list from functions.txt and generates
a properly structured Rust module file (api.rs) with all endpoints organized
by their namespace hierarchy.

Usage:
    python generate_api_rs.py
"""

import json
from collections import defaultdict
from pathlib import Path
from typing import Dict, List, Tuple

RUST_KEYWORDS = {'move', 'type', 'mod', 'use', 'fn', 'let', 'if', 'else', 'for', 
                 'while', 'loop', 'match', 'return', 'break', 'continue', 'const',
                 'static', 'struct', 'enum', 'trait', 'impl', 'self', 'super',
                 'crate', 'pub', 'extern', 'unsafe', 'async', 'await', 'dyn'}


def parse_function_path(func: str) -> Tuple[List[str], str]:
    """
    Parse a function path like 'ak.wwise.core.object.get' into:
    - module path: ['ak', 'wwise', 'core', 'object']
    - function name: 'get'
    """
    parts = func.split('.')
    return parts[:-1], parts[-1]


def escape_if_keyword(name: str) -> str:
    return f'r#{name}' if name in RUST_KEYWORDS else name


def build_module_tree(functions: List[str]) -> Dict:
    """
    Build a nested dictionary tree structure from function paths.
    
    Example:
    ['ak.soundengine.postEvent', 'ak.soundengine.getState']
    becomes:
    {'ak': {'soundengine': {'__functions': ['postEvent', 'getState']}}}
    """
    tree = {}
    
    for func in functions:
        module_path, func_name = parse_function_path(func)
        
        # Navigate to the correct position in the tree
        current = tree
        for module in module_path:
            if module not in current:
                current[module] = {}
            current = current[module]
        
        # Store the function at this level
        if '__functions' not in current:
            current['__functions'] = []
        current['__functions'].append((func_name, func))
    
    return tree


def generate_module_code(tree: Dict, indent_level: int = 0) -> List[str]:
    """
    Recursively generate Rust module code from the tree structure.
    
    Args:
        tree: The module tree dictionary
        indent_level: Current indentation level (multiples of 4 spaces)
    
    Returns:
        List of code lines
    """
    lines = []
    indent = '    ' * indent_level
    
    # Get all items from tree
    items = [(k, v) for k, v in tree.items() if k != '__functions']
    items.sort(key=lambda x: x[0]) 
    
    functions = tree.get('__functions', [])
    functions.sort(key=lambda x: x[0])
    
    for func_name, full_path in functions:
        escaped_name = escape_if_keyword(func_name)
        
        const_line = f'{indent}pub const {escaped_name}: &str = "{full_path}";'
        
        if len(const_line) > 100:
            lines.append(f'{indent}pub const {escaped_name}: &str =')
            lines.append(f'{indent}    "{full_path}";')
        else:
            lines.append(const_line)
    
    for module_name, subtree in items:
        if functions and not lines[-1] == '':
            lines.append('')
        
        lines.append(f'{indent}pub mod {module_name} {{')
        
        submodule_lines = generate_module_code(subtree, indent_level + 1)
        lines.extend(submodule_lines)
        
        lines.append(f'{indent}}}')
    
    return lines


def generate_api_file(functions: List[str], output_path: Path):
    """
    Generate the complete api.rs file.
    
    Args:
        functions: List of WAAPI function paths
        output_path: Path to write the api.rs file
    """
    # Build the module tree
    tree = build_module_tree(functions)
    
    lines = [
        '/// Wwise Authoring API endpoints',
        '#[allow(non_upper_case_globals)]',
        '#[allow(non_snake_case)]',
    ]
    
    module_lines = generate_module_code(tree, 0)
    lines.extend(module_lines)
    content = '\n'.join(lines) + '\n'
    output_path.write_text(content, encoding='utf-8')
    
    print(f"Generated {output_path} with {len(functions)} functions")


def generate_from_text(file_name, out_file_name, waapi_type):
    script_dir = Path(__file__).parent
    functions_file = script_dir / file_name
    output_file = script_dir.parent / 'src' / out_file_name
    
    if not functions_file.exists():
        print(f"Error: {functions_file} not found!")
        print("Run get_waapi_functions.py first to generate functions.txt")
        return 1
    
    with open(functions_file, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    functions = data.get(waapi_type, [])
    
    if not functions:
        print("Error: No functions found in functions.txt")
        return 1
    
    generate_api_file(functions, output_file)


if __name__ == '__main__':
    generate_from_text("waapi_functions.txt","waapi_function_api.rs", 'functions')
    generate_from_text("waapi_topics.txt", "waapi_topics_api.rs", 'topics')
