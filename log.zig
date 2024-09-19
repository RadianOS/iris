const std = @import("std");

const LogMode = enum {
    Info,
    Error,
    Warning,
    Note,
    Debug,
};

pub fn logGeneric(msg: []const u8) void {
    std.debug.print(" : {}\n", .{msg});
}

pub fn logCore(msg: []const u8, mode: LogMode) void {
    const prefixText = switch (mode) {
        LogMode.Info => "✓",
        LogMode.Error => "✗",
        LogMode.Warning => "!",
        LogMode.Note => "Note",
        LogMode.Debug => "Debug",
    };

    const coloredPrefix = applyColor(prefixText, mode);
    std.debug.print("({}) : {}\n", .{coloredPrefix, msg});
}

fn applyColor(text: []const u8, mode: LogMode) []const u8 {
    switch (mode) {
        LogMode.Info => return std.fmt.format("{s}", .{text}).bright_green(),
        LogMode.Error => return std.fmt.format("{s}", .{text}).bright_red(),
        LogMode.Warning => return std.fmt.format("{s}", .{text}).bright_yellow(),
        LogMode.Note => return std.fmt.format("{s}", .{text}).bright_yellow(),
        LogMode.Debug => return std.fmt.format("{s}", .{text}).bright_magenta(),
    }
}

macro_rules! log {
    ($mode:expr, $msg:expr) => ({
        logCore($msg, $mode);
    });
}

pub fn info(msg: []const u8) void {
    log!(LogMode.Info, msg);
}

pub fn error(msg: []const u8) void {
    log!(LogMode.Error, msg);
}

pub fn warning(msg: []const u8) void {
    log!(LogMode.Warning, msg);
}

pub fn note(msg: []const u8) void {
    log!(LogMode.Note, msg);
}

pub fn debug(msg: []const u8) void {
    log!(LogMode.Debug, msg);
}
