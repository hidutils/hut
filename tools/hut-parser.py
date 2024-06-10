#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
#
# Helper script to convert the jinja file into our hut.rs file
#
# Use as:
#     $ python3 ./tools/hut-parser.py src/hut.rs.jinja > src/hut.rs && cargo build
#

from pathlib import Path
from dataclasses import dataclass
import argparse
import enum
import os
import re
import sys
import functools
import json
import jinja2
import jinja2.environment


def sanitize(s):
    s = re.sub(r"[-‐\t: \\|/.\n,()]", "", s)
    s = re.sub(r"[+]", "Plus", s)
    s = re.sub(r"[-]", "Minus", s)
    numbers = {
        "1": "One",
        "2": "Two",
        "3": "Three",
        "4": "Four",
        "5": "Five",
        "6": "Six",
        "7": "Seven",
        "8": "Eight",
        "9": "Nine",
        "0": "Zero",
    }
    s = functools.reduce(
        lambda s, kv: re.sub(rf"^({kv[0]})", kv[1], s), numbers.items(), s
    )
    return s


def sanitize_keyboard(s):
    map = {
        "!": "ExclamationMark",
        "@": "At",
        "/": "Slash",
        "#": "Hash",
        "$": "Dollar",
        "%": "Percent",
        "^": "Caret",
        "&": "Ampersand",
        "*": "Star",
        "[": "OpenBracket",
        "]": "CloseBracket",
        "{": "OpenBrace",
        "}": "CloseBrace",
        "(": "OpenParenthesis",
        ")": "CloseParenthesis",
        "'": "SingleQuote",
        '"': "DoubleQuote",
        "<": "LessThan",
        ">": "GreaterThan",
        " and ": "And",
        " ": "",
        ",": "Comma",
        "-": "Minus",
        "+": "Plus",
        "|": "Pipe",
        "?": "QuestionMark",
        "=": "Equals",
        "\\": "Backslash",
        "~": "Tilde",
        ";": "Semicolon",
        ":": "Colon",
        ".": "Period",
    }
    s = functools.reduce(lambda s, kv: s.replace(kv[0], kv[1]), map.items(), s)

    numbers = {
        "1": "One",
        "2": "Two",
        "3": "Three",
        "4": "Four",
        "5": "Five",
        "6": "Six",
        "7": "Seven",
        "8": "Eight",
        "9": "Nine",
        "0": "Zero",
    }
    s = functools.reduce(
        lambda s, kv: re.sub(rf"^({kv[0]})", kv[1], s), numbers.items(), s
    )
    return s


class UsageType(enum.StrEnum):
    LINEAR_CONTROL = "LC"
    ON_OFF_CONTROL = "OOC"
    MOMENTARY_CONTROL = "MC"
    ONE_SHOT_CONTROL = "OSC"
    RETRIGGER_CONTROL = "RTC"
    SELECTOR = "Sel"
    STATIC_VALUE = "SV"
    STATIC_FLAG = "SF"
    DYNAMIC_VALUE = "DV"
    DYNAMIC_FLAG = "DF"
    NAMED_ARRAY = "NAry"
    APPLICATION_COLLECTION = "CA"
    LOGICAL_COLLECTION = "CL"
    PHYSICAL_COLLECTION = "CP"
    USAGE_SWITCH = "US"
    USAGE_MODIFIER = "UM"
    BUFFERED_BYTES = "BufferedBytes"


class UsagePageType(enum.StrEnum):
    DEFINED = "Defined"
    GENERATED = "Generated"


@dataclass
class UsageId:
    printable: str  # The string as it was in the data file
    name: str  # Name of the usage sanitized to be a valid identifier
    value: int
    usage_types: list[UsageType]


@dataclass
class UsagePage:
    printable: str
    name: str
    value: int
    usages: list[UsageId]
    usage_page_type: UsagePageType
    name_prefix: str | None


def parse_usage_id(u) -> UsageId:
    # Some of the strings have \_ in them for whatever reason...
    printable = u["Name"].replace("\\", "")
    name = sanitize(printable)
    value = int(u["Id"])
    types = [UsageType(k) for k in u["Kinds"]]
    usage = UsageId(printable=printable, name=name, value=value, usage_types=types)
    return usage


def parse_usage_page(up) -> UsagePage:
    printable = up["Name"]
    name = sanitize(printable)
    value = int(up["Id"])
    usages = sorted([parse_usage_id(u) for u in up["UsageIds"]], key=lambda u: u.value)
    uptype = UsagePageType(up["Kind"])
    name_prefix = None
    if uptype == UsagePageType.GENERATED:
        name_prefix = up["UsageIdGenerator"]["NamePrefix"]
        if name_prefix.lower() == "enum":
            name_prefix += "erate"
    usage_page = UsagePage(
        printable=printable,
        name=name,
        value=value,
        usages=usages,
        usage_page_type=uptype,
        name_prefix=name_prefix,
    )
    return usage_page


def parse_data_files(datadir: Path):
    usage_pages = []

    for datafile in datadir.glob("*.json"):
        js = json.load(open(datafile, "r"))
        ups = [parse_usage_page(up) for up in js["UsagePages"]]
        usage_pages.extend(ups)

    # For some reason the Unicode usage page isn't in the JSON but
    # it is described on page 213 of the HUT 1.5 document. Let's insert
    # it manually.
    unicode_page = UsagePage(
        printable="Unicode",
        name="Unicode",
        value=0x10,
        usages=[],
        usage_page_type=UsagePageType.GENERATED,
        name_prefix="codepoint",
    )
    usage_pages.append(unicode_page)

    return sorted(usage_pages, key=lambda up: up.value)


def generate_source(
    usage_pages: list[UsagePage], template: str
) -> jinja2.environment.TemplateStream:
    data = {}
    data["usage_pages"] = usage_pages

    loader: jinja2.BaseLoader
    if template == "-":
        loader = jinja2.FunctionLoader(lambda _: sys.stdin.read())
        filename = "<stdin>"
    else:
        path = Path(template)
        assert path.exists(), f"Failed to find template {path}"
        filename = path.name
        loader = jinja2.FileSystemLoader(os.fspath(path.parent))

    env = jinja2.Environment(
        loader=loader,
        trim_blocks=True,
        lstrip_blocks=True,
    )
    jtemplate = env.get_template(filename)
    return jtemplate.stream(data)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", default="-")
    parser.add_argument(
        "--datadir",
        type=Path,
        default=Path("data"),
        help="Path to the directory containing the HUT json files",
    )
    parser.add_argument("template", type=Path, help="The jinja template file")
    args = parser.parse_args()
    assert args.datadir.exists()
    assert args.template.exists()

    hut = parse_data_files(args.datadir)

    stream = generate_source(usage_pages=hut, template=args.template)
    file = sys.stdout if args.output == "-" else open(args.output, "w")
    stream.dump(file)
