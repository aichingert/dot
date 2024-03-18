const std = @import("std");

pub fn build(b: *std.Build) void {
    const exe = b.addExecutable(.{
        .name = "cli-tool",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = std.zig.CrossTarget{ .os_tag = .linux, .cpu_arch = .x86_64 },
    });

    b.installArtifact(exe);
}
