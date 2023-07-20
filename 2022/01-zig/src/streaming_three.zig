const std = @import("std");
const out = std.io.getStdOut().writer();

pub fn main() !void {
    var args = std.process.args();
    const name = args.next().?;
    var file_path = args.next() orelse {
        std.debug.print("usage: {s} <file_path>\n", .{name});
        return;
    };
    var file = try std.fs.cwd().openFile(file_path, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var stream = buf_reader.reader();
    var buf: [1024]u8 = undefined;
    var fixed_buffer = std.io.fixedBufferStream(&buf);

    var sum: u32 = 0;
    var nums = [_]u32{0} ** 3;

    while (stream.streamUntilDelimiter(fixed_buffer.writer(), '\n', null)) : ({
        fixed_buffer.reset();
    }) {
        const line = buf[0..fixed_buffer.pos];
        if (line.len == 0) {
            replace_smallest(3, u32, &nums, sum);
            sum = 0;
        } else {
            sum += try std.fmt.parseInt(u32, line, 10);
        }
    } else |err| {
        if (err != error.EndOfStream) {
            return err;
        }
    }
    replace_smallest(3, u32, &nums, sum);
    std.sort.pdq(u32, &nums, {}, std.sort.desc(u32));

    try out.print("{}\n", .{nums[0]});
    var top_three: u32 = 0;
    for (nums[0..3]) |num| {
        top_three += num;
    }
    try out.print("{}\n", .{top_three});
}

fn replace_smallest(comptime N: usize, comptime T: type, arr: []T, val: T) void {
    var smallest: usize = 0;
    for (arr[0..N], 0..) |elem, i| {
        if (elem < arr[smallest]) {
            smallest = i;
        }
    }
    if (val > arr[smallest]) {
        arr[smallest] = val;
    }
}
