const std = @import("std");

const allocator = std.heap.page_allocator;

fn containsTerm(line: []const u8, term: []const u8) ?usize {
    var loc: usize = 0;
    var end: usize = term.len;

    while (end + 1 < line.len) {
        if (std.mem.eql(u8, line[loc..end], term)) {
            return loc;
        }

        loc += 1;
        end += 1;
    }

    return null;
}

fn searchDir(ident: []const u8, term: []const u8) !void {
    var dir = try std.fs.cwd().openIterableDir(ident, .{});
    defer dir.close();

    var iter = dir.iterate();

    while (try iter.next()) |entry| {
        const path = try concatDir(ident, entry.name);

        if (entry.kind == .directory) {
            try searchDir(path, term);
            continue;
        }
        var file = try std.fs.cwd().openFile(path, .{ .mode = std.fs.File.OpenMode.read_only });
        defer file.close();

        var buf_reader = std.io.bufferedReader(file.reader());
        var in_stream = buf_reader.reader();
        var buf: [1024]u8 = undefined;

        while (in_stream.readUntilDelimiterOrEof(&buf, '\n') catch {
            continue;
        }) |line| {
            if (containsTerm(line, term)) |loc| {
                std.debug.print("Dir: {s}\n", .{ident});
                std.debug.print("   - file: {s} | {any}\n", .{ entry.name, entry.kind });
                std.debug.print("       - {d}\n", .{loc});
            }
        }
    }
}

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

    const term = args.next() orelse {
        std.debug.print("ERROR: you have to provide a search term\n", .{});
        return;
    };

    try searchDir(".", term);
}
