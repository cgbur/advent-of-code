const std = @import("std");

const input = @embedFile("input");
const out = std.io.getStdOut().writer();

const example_input =
    \\1000
    \\2000
    \\3000
    \\
    \\4000
    \\
    \\5000
    \\6000
    \\
    \\7000
    \\8000
    \\9000
    \\
    \\10000
;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    var nums = std.ArrayList(u32).init(arena.allocator());

    var group_it = std.mem.tokenizeSequence(u8, input, "\n\n");
    // var it = std.mem.tokenizeSequence(u8, example_input, "\n\n");
    while (group_it.next()) |line| {
        var sum: u32 = 0;
        var word_it = std.mem.tokenizeAny(u8, line, "\n");
        while (word_it.next()) |word| {
            sum += try std.fmt.parseInt(u32, word, 10);
        }
        try nums.append(sum);
    }

    std.sort.pdq(u32, nums.items, {}, std.sort.desc(u32));

    try out.print("{}\n", .{nums.items[0]});
    var top_three: u32 = 0;
    for (nums.items[0..3]) |num| {
        top_three += num;
    }
    try out.print("{}\n", .{top_three});
}
