#!/usr/bin/env python3

import argparse
import subprocess


def styled_print(message):
    print(f"[👷🏼‍♂️ Bob]: {message}")


def cargo_build(package=None, release=False):
    args = ["cargo", "build"]
    if package:
        args.append(f"--package={package}")
    if release:
        args.append("--release")
    subprocess.run(args)


def build_cockpit_frontend():
    styled_print("Building frontend...")
    subprocess.run(["npm", "install"], cwd="cockpit/frontend/web")
    subprocess.run(["npm", "run", "build"], cwd="cockpit/frontend/web")


def build_cockpit_backend():
    styled_print("Building backend...")
    # TODO: Add ability to use a --release flag for build subcommand.
    cargo_build("cockpit-backend")


def build_runtime():
    styled_print("Building runtime...")
    cargo_build("runtime")


def build_carburetor():
    styled_print("Building carburetor...")
    cargo_build("carburetor")


def build_lib():
    styled_print("Building linkage lib...")
    subprocess.run(["npm", "install"], cwd="lib/linkage-node")
    subprocess.run(["npm", "run", "build"], cwd="lib/linkage-node")


def build_lib_examples():
    styled_print("Building linkage lib examples...")
    subprocess.run(["npm", "link"], cwd="lib/linkage-node")
    subprocess.run(
        ["npm", "link", "@impossiblerobotics/linkage", "--save"],
        cwd="examples/lib/linkage-node",
    )
    subprocess.run(["npm", "run", "build"], cwd="examples/lib/linkage-node")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Build script for the many moving parts of Linkage",
        epilog="Koen Westendorp & Bauke Westendorp, 2023",
    )

    subparsers = parser.add_subparsers(title="subarguments", required=True)
    build = subparsers.add_parser("build", help="build the moving parts of linkage")
    # deploy = subparsers.add_parser("deploy", help="deploy the moving parts of linkage")
    # test = subparsers.add_parser("test", help="test the moving parts of linkage")

    build.add_argument(
        "part",
        help="the part of linkage to build",
        choices=[
            "all",
            "cockpit",
            "cockpit-frontend",
            "cockpit-backend",
            "runtime",
            "carburetor",
            "lib",
            "lib-examples",
            "lib-examples-only",
        ],
    )
    args = parser.parse_args()

    match args.part:
        case "all":
            styled_print("Building all parts...")
            build_cockpit_frontend()
            cargo_build()
            build_lib()
            build_lib_examples()

        case "cockpit":
            styled_print("Building cockpit frontend and backend...")
            build_cockpit_frontend()
            build_cockpit_backend()

        case "cockpit-frontend":
            build_cockpit_frontend()

        case "cockpit-backend":
            build_cockpit_backend()

        case "runtime":
            build_runtime()

        case "carburetor":
            build_carburetor()

        case "lib-examples":
            styled_print("Building linkage lib and its examples...")
            build_lib()
            build_lib_examples()

        case "lib-examples-only":
            build_lib_examples()

        case "lib":
            build_lib()

        case unknown:
            styled_print("ERROR: Part '{unknown}' not recognized")

    styled_print("Done!")
    exit(0)
