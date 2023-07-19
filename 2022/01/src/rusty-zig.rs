//! the rust equal to the zig version
const INPUT: &str = include_str!("input");
// pub fn main() !void {
//     var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
//     defer arena.deinit();
//     const allocator = arena.allocator();
//     var nums = std.ArrayList(u32).init(allocator);
//
//     var it = std.mem.tokenizeSequence(u8, input, "\n\n");
//     // var it = std.mem.tokenizeSequence(u8, example_input, "\n\n");
//     while (it.next()) |line| {
//         var sum: u32 = 0;
//         var word_it = std.mem.tokenizeAny(u8, line, "\n");
//         while (word_it.next()) |word| {
//             sum += try std.fmt.parseInt(u32, word, 10);
//         }
//         try nums.append(sum);
//     }
//
//     std.sort.pdq(u32, nums.items, {}, std.sort.desc(u32));
//     var writ = std.io.getStdOut().writer();
//     var buf_writer = std.io.bufferedWriter(writ);
//     defer buf_writer.flush() catch unreachable;
//
//     try buf_writer.writer().print("{}\n", .{nums.items[0]});
//     var top_three: u32 = 0;
//     for (nums.items[0..3]) |num| {
//         top_three += num;
//     }
//     try buf_writer.writer().print("{}\n", .{top_three});
// }
fn main() -> std::io::Result<()> {
    let mut counts = vec![];

    for group in INPUT.split("\n\n") {
        let mut sum = 0;
        for line in group.split("\n") {
            sum += line.parse::<u32>().unwrap_or(0);
        }
        counts.push(sum);
    }

    counts.sort_unstable_by(|a, b| b.cmp(a));

    println!("{}", counts[0]);

    let mut top_three = 0;
    for count in &counts[0..3] {
        top_three += count;
    }
    println!("{}", top_three);

    Ok(())
}
