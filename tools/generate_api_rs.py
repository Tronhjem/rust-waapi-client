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

# Rust keywords that need to be escaped with r#
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
    """Add r# prefix if the name is a Rust keyword"""
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
    items.sort(key=lambda x: x[0])  # Sort alphabetically
    
    # Get functions at this level
    functions = tree.get('__functions', [])
    functions.sort(key=lambda x: x[0])  # Sort alphabetically
    
    # Generate const declarations for functions at this level
    for func_name, full_path in functions:
        escaped_name = escape_if_keyword(func_name)
        
        # Check if the line will be too long (> 100 chars is typical Rust convention)
        const_line = f'{indent}pub const {escaped_name}: &str = "{full_path}";'
        
        if len(const_line) > 100:
            # Split into multiple lines
            lines.append(f'{indent}pub const {escaped_name}: &str =')
            lines.append(f'{indent}    "{full_path}";')
        else:
            lines.append(const_line)
    
    # Generate submodules
    for module_name, subtree in items:
        # Add blank line before submodule if we already have content
        if functions and not lines[-1] == '':
            lines.append('')
        
        lines.append(f'{indent}pub mod {module_name} {{')
        
        # Recursively generate submodule content
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
    
    # Start with file header
    lines = [
        '/// Wwise Authoring API endpoints',
        '#[allow(non_upper_case_globals)]',
        '#[allow(non_snake_case)]',
    ]
    
    # Generate the module code
    module_lines = generate_module_code(tree, 0)
    lines.extend(module_lines)
    
    # Write to file
    content = '\n'.join(lines) + '\n'
    output_path.write_text(content, encoding='utf-8')
    
    print(f"Generated {output_path} with {len(functions)} functions")


def generate_from_text(file_name, out_file_name, waapi_type):
    script_dir = Path(__file__).parent
    functions_file = script_dir / file_name
    output_file = script_dir.parent / 'src' / out_file_name
    
    # Check if functions.txt exists
    if not functions_file.exists():
        print(f"Error: {functions_file} not found!")
        print("Run get_waapi_functions.py first to generate functions.txt")
        return 1
    
    # Load functions from JSON file
    with open(functions_file, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    functions = data.get(waapi_type, [])
    
    if not functions:
        print("Error: No functions found in functions.txt")
        return 1
    
    # Generate the api.rs file
    generate_api_file(functions, output_file)

def main():
    result = generate_from_text("waapi_functions.txt","waapi_function_api.rs", 'functions')
    result = generate_from_text("waapi_topics.txt", "waapi_topics_api.rs", 'topics')
    
    return 0


if __name__ == '__main__':
    exit(main())
