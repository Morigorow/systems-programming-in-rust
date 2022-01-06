// 1st path : マッチした行にタグ付け
// 2nd path : それぞれのタグの前後からctx_lines=2行を集める
fn main() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
    Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek
    through millions of pages?";
    // tagsにマッチした各行の番号を入れる
    let mut tags: Vec<usize> = vec![];
    // ctxにコンテクスト行を格納したマッチごとのベクターを入れる
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    // linesのイテレーションでマッチがあった行番号を記録
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);
            // Vec::with_capacity(n)でｎ個の要素のための空間を予約
            let v = Vec::with_capacity(2*ctx_lines + 1);    // 17行目のctx定義から推論しているため型シグネチャは不要。
            ctx.push(v);
        }
    }

    if tags.is_empty() {        // マッチがない場合は処理を早期終了
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }
    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}