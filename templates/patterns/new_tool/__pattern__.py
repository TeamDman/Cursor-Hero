def gather_global_variables() -> dict[str,str]:
    crate_name = input("crate_name")
    assert crate_name, "crate_name is required"
    crate_name_pascal = pascal(crate_name)
    assert not crate_name.endswith("tool")
    return {
        "crate_name": crate_name,
        "crate_name_pascal": crate_name_pascal,
    }

def pascal(snake_string: str) -> str:
    return ''.join(word.capitalize() for word in snake_string.split('_'))

