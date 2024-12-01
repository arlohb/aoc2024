const std = @import("std");

fn parseInput() ![2]std.ArrayList(u32) {
    const file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var left = std.ArrayList(u32).init(std.heap.page_allocator);
    var right = std.ArrayList(u32).init(std.heap.page_allocator);

    const buffer = try file.readToEndAlloc(std.heap.page_allocator, std.math.maxInt(usize));
    var lines = std.mem.split(u8, buffer, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) continue;

        var parts = std.mem.split(u8, line, "   ");
        const a = try std.fmt.parseInt(u32, parts.next().?, 10);
        const b = try std.fmt.parseInt(u32, parts.next().?, 10);

        try left.append(a);
        try right.append(b);
    }

    return [2]std.ArrayList(u32){ left, right };
}

fn part1() !void {
    const input = try parseInput();
    var left = input[0];
    defer left.deinit();
    var right = input[1];
    defer right.deinit();

    std.mem.sort(u32, left.items, {}, comptime std.sort.asc(u32));
    std.mem.sort(u32, right.items, {}, comptime std.sort.asc(u32));

    var sum: u32 = 0;

    for (left.items, right.items) |a, b| {
        const a_i: i32 = @intCast(a);
        const b_i: i32 = @intCast(b);

        const diff = @abs(a_i - b_i);
        sum += diff;
    }

    std.debug.print("{d}\n", .{sum});
}

fn freq(xs: []const u32) !std.AutoHashMap(u32, u32) {
    var map = std.AutoHashMap(u32, u32).init(std.heap.page_allocator);

    for (xs) |x| {
        const count = map.get(x) orelse 0;
        try map.put(x, count + 1);
    }

    return map;
}

fn part2() !void {
    const input = try parseInput();
    var left = input[0];
    defer left.deinit();
    var right = input[1];
    defer right.deinit();

    var counts = try freq(right.items);
    defer counts.deinit();
    var score: u32 = 0;

    for (left.items) |x| {
        score += x * (counts.get(x) orelse 0);
    }

    std.debug.print("{d}\n", .{score});
}

pub fn main() !void {
    try part1();
    try part2();
}
