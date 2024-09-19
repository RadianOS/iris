const std = @import("std");
const yazap = @import("yazap");

const VERSION_TEXT =
    \\d8b       d8b                    Iris Package Manager         
    \\Y8P       Y8P                 -------------------------
    \\                              Iris v1.0.0
    \\888888d888888.d8888b          Copyright (C) 2024-2025 RadianOS Development Team
    \\888888P\"  88888K         
    \\888888    888\"Y8888b.         Copyright (C) 2024-2025 Atiksh Sharma
    \\888888    888     X88         This program may be freely redistributed under
    \\888888    888 88888P'           the terms of the GNU General Public License.
;

fn print_version() void {
    std.debug.print("{}", .{VERSION_TEXT});
}

fn confirm(prompt: []const u8) bool {
    var input: [256]u8 = undefined;
    while (true) {
        std.debug.print("{} [Y/N] ", .{prompt});
        const stdout = std.io.getStdOut().writer();
        stdout.flush() catch {};

        const readResult = std.io.getStdIn().readUntilDelimiterOrEof(input[0..], '\n');
        if (readResult.isError()) return false;

        const trimmed = std.mem.trim(u8, input, ' ');
        if (std.mem.eql(u8, trimmed, "Y") or std.mem.eql(u8, trimmed, "YES")) return true;
        if (std.mem.eql(u8, trimmed, "N") or std.mem.eql(u8, trimmed, "NO")) return false;

        std.debug.print("Invalid input. Please enter 'Y' or 'N'.\n", .{});
    }
}

fn main() void {
    var parser = yazap.Parser.init();
    parser.add_flag("version", "Show version information");
    parser.add_subcommand("install", "Install a package")
        .add_argument("pkg", "Package to install");
    parser.add_subcommand("remove", "Remove packages")
        .add_argument("pkgs", "Packages to remove")
        .add_option("force", "Force removal", false)
        .add_option("yes", "Automatic yes to prompts", false);
    parser.add_subcommand("search", "Search for packages")
        .add_argument("terms", "Search terms")
        .add_option("all", "Search all available packages", false);
    parser.add_subcommand("query", "Query package information")
        .add_argument("terms", "Terms to query")
        .add_option("details", "Show detailed information", false);
    parser.add_subcommand("list", "List installed packages");
    parser.add_subcommand("upgrade", "Upgrade system packages");
    parser.add_subcommand("sync", "Sync repositories");
    parser.add_subcommand("add-repo", "Add a repository")
        .add_argument("repo", "Repository to add")
        .add_option("update", "Update repository list after adding", false);
    parser.add_subcommand("downgrade", "Downgrade packages")
        .add_argument("pkgs", "Packages to downgrade")
        .add_option("version", "Version to downgrade to", null);
    parser.add_subcommand("resume", "Resume operations")
        .add_option("all", "Resume all operations", false)
        .add_option("id", "Operation ID to resume", null);

    const args = parser.parse(std.os.args());
    if (args.isError()) {
        std.debug.print("Error parsing arguments: {}\n", .{args});
        return;
    }

    if (args.version) {
        print_version();
        return;
    }

    switch (args.subcommand) {
        "install" => |install| {
            const pkg = install.pkg;
            std.debug.print("Installing package: {}\n", .{pkg});
        },
        "remove" => |remove| {
            const pkgs = remove.pkgs;
            const force = remove.force;
            const yes = remove.yes;
            if (!force and !yes and !confirm("Are you sure you want to remove the packages")) {
                std.debug.print("Removal aborted.\n", .{});
                return;
            }
            std.debug.print("Removing packages: {:?}\n", .{pkgs});
            if (force) {
                std.debug.print("Force removal enabled.\n", .{});
            }
        },
        "search" => |search| {
            std.debug.print("Searching packages: {:?}\n", .{search.terms});
            if (search.all) {
                std.debug.print("Searching all available packages.\n", .{});
            }
        },
        "query" => |query| {
            std.debug.print("Querying packages: {:?}\n", .{query.terms});
            if (query.details) {
                std.debug.print("Detailed information requested.\n", .{});
            }
        },
        "list" => {
            std.debug.print("Listing packages\n", .{});
        },
        "upgrade" => {
            if (!confirm("Are you sure you want to upgrade the system packages")) {
                std.debug.print("Upgrade aborted.\n", .{});
                return;
            }
            std.debug.print("Upgrading system packages\n", .{});
        },
        "sync" => {
            if (!confirm("Are you sure you want to sync the repositories")) {
                std.debug.print("Sync aborted.\n", .{});
                return;
            }
            std.debug.print("Syncing repositories\n", .{});
        },
        "add-repo" => |add_repo| {
            std.debug.print("Adding repository: {}\n", .{add_repo.repo});
            if (add_repo.update) {
                if (!confirm("Are you sure you want to update the repository list")) {
                    std.debug.print("Update aborted.\n", .{});
                    return;
                }
                std.debug.print("Updating repository list.\n", .{});
            }
        },
        "downgrade" => |downgrade| {
            const pkgs = downgrade.pkgs;
            const version = downgrade.version;
            if (!downgrade.force and !confirm("Are you sure you want to downgrade the packages")) {
                std.debug.print("Downgrade aborted.\n", .{});
                return;
            }
            std.debug.print("Downgrading packages: {:?}\n", .{pkgs});
            if (version) |ver| {
                std.debug.print("Downgrading to version: {}\n", .{ver});
            }
        },
        "resume" => |resume_operation| {
            if (!resume .all and resume .id == null and !confirm("Are you sure you want to resume the operation")) {
                std.debug.print("Resume aborted.\n", .{});
                return;
            }
            if (resume .all) {
                std.debug.print("Resuming all paused operations.\n", .{});
            } else if (resume .id) |id| {
                std.debug.print("Resuming operation with ID: {}\n", .{id});
            } else {
                std.debug.print("Resuming operation\n", .{});
            }
        },
        else => {
            std.debug.print("No valid subcommand specified.\n", .{});
        },
    }
}
