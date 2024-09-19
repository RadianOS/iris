const std = @import("std");
const http = @import("std").http;
const fs = @import("std").fs;
const tar = @import("tar.zig");
const xz = @import("xz.zig");
const indicatif = @import("indicatif.zig");
const confirm = @import("confirm.zig");

fn download_package(url: []const u8, output_path: []const u8) !void {
    const allocator = std.heap.page_allocator;

    const response = try http.get(url);
    const total_size = response.content_length;

    const file = try fs.File.openWrite(allocator, output_path, fs.File.write_mode());
    defer file.close();

    var pb = indicatif.ProgressBar.init(total_size);
    pb.set_style(indicatif.ProgressStyle.default_bar()
        .template("{msg} [{bar:40}] {percent}%")
        .progress_chars("##-"));

    var downloaded: u64 = 0;
    var buffer: [8 * 1024]u8 = undefined;

    while (true) {
        const bytes_read = try response.read(buffer);
        if (bytes_read == 0) break;
        try file.writeAll(buffer[0..bytes_read]);
        downloaded += bytes_read;

        pb.set_message("Downloading...");
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");
}

fn extract_package(tar_xz_path: []const u8, output_dir: []const u8) !void {
    const file = try fs.File.openRead(tar_xz_path);
    const decompressor = try xz.Decompressor.init(file);
    const archive = try tar.Archive.init(decompressor);

    var pb = indicatif.ProgressBar.init(archive.total_entries());
    pb.set_style(indicatif.ProgressStyle.default_bar()
        .template("{msg} [{bar:40}] {percent}%")
        .progress_chars("##-"));

    while (true) {
        const entry = try archive.next_entry();
        if (entry == null) break;

        try entry.unpack(output_dir);
        pb.set_message("Extracting...");
        pb.inc(1);
    }

    pb.finish_with_message("Extraction complete");
}

pub fn install_packages(pkgs: []const []const u8, _force: bool) !void {
    for (pkgs) |pkg| {
        std.debug.print("Package: {}\n", .{pkg});
        std.debug.print("Total Installed Size: 2.70 MiB\n\n");

        if (!confirm(":: Proceed with installation? ")) {
            std.debug.print("Installation for {} aborted.\n", .{pkg});
            continue;
        }

        const url = "https://raw.githubusercontent.com/RadianOS/zephpkgs/main/".++pkg;
        const output_path = "/tmp/".++pkg;

        try download_package(url, output_path);
        std.debug.print("Extracting {}...\n", .{pkg});
        try extract_package(output_path, "/home/rudy");
        std.debug.print("{} installed successfully.\n", .{pkg});
    }

    std.debug.print("All installations complete.\n", .{});
}
