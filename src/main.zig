const std = @import("std");

fn searchDir(ident: []const u8) !void {
    std.debug.print("Dir: {s}\n", .{ident});

    var dir = try std.fs.cwd().openIterableDir(ident, .{});
    defer dir.close();

    var iter = dir.iterate();

    while (try iter.next()) |entry| {
        const path = try concatDir(ident, entry.name);

        if (entry.kind == .directory) {
            try searchDir(path);
        } else if (entry.kind == .file) {
            var file = try std.fs.cwd().openFile(path, .{ .mode = std.fs.File.OpenMode.read_only });

            var buf_reader = std.io.bufferedReader(file.reader());
            var in_stream = buf_reader.reader();
            var buf: [1024]u8 = undefined;

            std.debug.print("    file: {s} | {any}\n", .{ entry.name, entry.kind });
            while (in_stream.readUntilDelimiterOrEof(&buf, '\n') catch {
                continue;
            }) |line| {
                _ = line;
            }

            file.close();
        }
    }
}

const allocator = std.heap.page_allocator;

fn concatDir(f: []const u8, s: []const u8) ![]const u8 {
    var res = try allocator.alloc(u8, f.len + s.len + 1);
    std.mem.copy(u8, res[0..], f);
    res[f.len] = '/';
    std.mem.copy(u8, res[f.len + 1 ..], s);
    return res;
}

pub fn main() !void {
    var args = std.process.args();
    _ = args.skip();

    _ = args.next() orelse {
        std.debug.print("ERROR: you have to provide a search term\n", .{});
        return;
    };

    try searchDir(".");
}
