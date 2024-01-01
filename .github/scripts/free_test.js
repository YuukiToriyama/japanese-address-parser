const {Parser} = require("../../pkg");
const readline = require("readline");
const fs = require("fs");

const reader = readline.createInterface({
    input: process.stdin
});

const stream = fs.createWriteStream("test_result.md");
stream.write("### テスト結果\n");
stream.write("<details>\n\n");
stream.write("|ステータス|入力|結果|\n");
stream.write("|-|-|-|\n");

const parser = new Parser();
const count = {
    total: 0,
    success: 0
};
reader.on("line", async line => {
    count.total++;
    const result = await parser.parse(line);
    if (result.error === undefined) {
        count.success++;
    }
    const status = result.error === undefined ? ":white_check_mark:" : ":x:";
    stream.write(`|${status}|${line}|${JSON.stringify(result.address)}|\n`);
});

process.on("exit", () => {
    stream.write([
        "\n</details>\n",
        "### 統計",
        `- ケース数: ${count.total}`,
        `- 成功数: ${count.success}`,
        `- 失敗数: ${count.total - count.success}`
    ].join("\n"));
});