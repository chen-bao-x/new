use colored::Colorize;
use prettytable::*;

pub fn usage() -> String {
    let mut table = Table::new();

    // Add a row per time   format!("{} {}", "new".bright_yellow(), "filename".underline())
    table.add_row(row![
        "new filename.txt".bright_yellow(),
        "# 在当前目录创建文件",
    ]);
    table.add_row(row![
        "new foldername/".bright_yellow(),
        "# 在当前目录创建文件夹, 末尾一定要有 \"/\"",
    ]);
    table.add_row(row![
        "new folder_1/filename.txt".bright_yellow(),
        "# 在当前目录创建 文件夹/文件",
    ]);

    let format = format::FormatBuilder::new()
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new(' ', ' ', ' ', ' '),
        )
        .padding(4, 0)
        .build();

    table.set_format(format);

    // Print the table to stdout
    // table.printstd();
    return table.to_string();
}
