const std = @import("std");
const cli = @import("cli.zig");

fn confirmPrompt(prompt: []const u8) !bool {
    var input: [256]u8 = undefined;
    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();

    while (true) {
        try(stdout.print("{} (Y/N): ", .{prompt}));
        try(stdout.flush());
        const len = try(stdin.readUntilDelimiterOrEof(&input, '\n'));
        const trimmed = input[0..len].trim();
        const upper = trimmed.toUpper();

        if (upper == "Y" or upper == "YES") return true;
        if (upper == "N" or upper == "NO") return false;

        try(stdout.print("Invalid input. Please enter 'Y' or 'N'.\n", .{}));
    }
}

pub fn cmdHandler(cli: *cli.Cli) void {
    switch (cli.operation) {
        cli.Operations.Install => |install| {
            if (!install.force and !confirmPrompt("Are you sure you want to install the packages")) {
                std.debug.print("Installation aborted.\n", .{});
                return;
            }
            std.debug.print("Installing packages: {:?}\n", .{install.pkgs});
        },
        cli.Operations.Remove => |remove| {
            if (!remove.force and !remove.yes and !confirmPrompt("Are you sure you want to remove the packages")) {
                std.debug.print("Removal aborted.\n", .{});
                return;
            }
            std.debug.print("Removing packages: {:?}\n", .{remove.pkgs});
        },
        cli.Operations.Downgrade => |downgrade| {
            if (!downgrade.force and !confirmPrompt("Are you sure you want to downgrade the packages")) {
                std.debug.print("Downgrade aborted.\n", .{});
                return;
            }
            std.debug.print("Downgrading packages: {:?}\n", .{downgrade.pkgs});
            if (downgrade.version) |version| {
                std.debug.print("To version: {}\n", .{version});
            }
        },
        cli.Operations.AddRepo => |add_repo| {
            if (!add_repo.update and !confirmPrompt("Are you sure you want to add this repository")) {
                std.debug.print("Repository addition aborted.\n", .{});
                return;
            }
            std.debug.print("Adding repository: {}\n", .{add_repo.repo});
        },
        cli.Operations.Resume => |resume| {
            if (!resume.all and !confirmPrompt("Are you sure you want to resume the operation")) {
                std.debug.print("Resume aborted.\n", .{});
                return;
            }
            std.debug.print("Resuming operations with ID: {:?}\n", .{resume.id});
        },
        else => {},
    }
}
