const std = @import("std");

pub fn main() !void {
    var args = std.process.args();
    _ = args.skip();

    const term = args.next() orelse {
        std.debug.print("ERROR: you have to provide a search term\n", .{});
        return;
    };

    var dir = try std.fs.cwd().openIterableDir(".", .{});
    defer dir.close();

    var iter = dir.iterate();

    while (try iter.next()) |entry| {
        if (entry.kind == .directory) {
            std.debug.print("dir: {s}\n", .{entry.name});
            var subDir = try std.fs.cwd().openIterableDir(entry.name, .{});

            var subIter = subDir.iterate();
            while (try subIter.next()) |subEntry| {
                std.debug.print("   - {s} {?}\n", .{ subEntry.name, subEntry.kind });
            }

            subDir.close();
        }
        std.debug.print("{s}\n", .{entry.name});
    }

    std.debug.print("{s}\n", .{term});
}
