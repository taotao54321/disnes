//! 制御フロー解析。
//!
//! `NotCode` にしか到達しない制御フローを `NotCode` とする。
//! また、`Code` から一意に辿れる制御フローを `Code` とする。

use arrayvec::ArrayVec;

use crate::address::Address;
use crate::input::Input;
use crate::memory::OpSuccResolved;

use super::{Analysis, AnalysisKind};

pub(super) fn analyze(analysis: &mut Analysis, input: &Input) {
    analyze_notcode(analysis, input);
    analyze_code(analysis, input);
}

/// `NotCode` にしか到達しない制御フローを `NotCode` とする。
fn analyze_notcode(analysis: &mut Analysis, input: &Input) {
    // 以下のような制御フローグラフ G(V, E) を考える:
    //
    // * V は全ての論理アドレス、および特別な頂点 C からなる。
    //   C は仮想的な Code 頂点で、「どこかにあるコード」を表すと考えてよい。
    // * E は制御フローを表す辺集合。
    //   Code である頂点、または Unknown かつ命令取得不可な頂点は自己ループを持つ。
    //   Unknown かつ命令取得可な頂点は全ての後続アドレスへの辺を持つ。
    //   (後続アドレスが具体的に確定できない場合(後述)は C への辺とする)
    //   NotCode である頂点は辺を持たない。
    //
    // G の逆グラフ T を考える。このとき、T 上で入次数 0 の頂点は全て NotCode であるはず。
    // よって、T 上で入次数 0 の頂点から出る辺を全て消すと、
    // 新たに入次数 0 となった頂点も全て NotCode としてよい。
    // これを変化がなくなるまで繰り返せば、NotCode にしか到達しない制御フローが全て NotCode となる。
    // (トポロジカルソートにおける Kahn のアルゴリズムと同じ要領)
    //
    // 「後続アドレスが具体的に確定できる」ためには、以下のいずれかの条件を満たさねばならない:
    //
    // * 命令とその後続アドレスがともに同一バンク上にある。
    // * 後続アドレスが非固定バンク上にない。
    //
    // このルールはバンク切り替えを考慮したもの。
    // (後続アドレスが別の非固定バンク上にある場合、実際に参照されるバンクを確定できない)

    let (graph_t, mut in_degs) = make_transpose_graph(analysis, input);

    // 入次数 0 の頂点を全てスタックに入れる。
    // 仮想頂点 C を直接扱うことはないので、型は Address にしてしまう。
    let mut stack: Vec<_> = Address::all()
        .filter(|&addr| in_degs[usize::from(addr)] == 0)
        .collect();

    // スタックが空になるまで前述の操作を繰り返す。
    // この操作により仮想頂点 C が入次数 0 になることはないから、常に Address 型を使ってよい。
    while let Some(addr) = stack.pop() {
        assert_eq!(analysis[addr], AnalysisKind::NotCode);

        for &dst in &graph_t[usize::from(addr)] {
            in_degs[dst] -= 1;
            if in_degs[dst] == 0 {
                let dst = Address::new(u16::try_from(dst).unwrap());
                analysis[dst] = AnalysisKind::NotCode;
                stack.push(dst);
            }
        }
    }
}

/// グラフの隣接リスト表現。インデックス 0x10000 は特別な頂点 C を表す。
type Graph = Box<[Vec<usize>; 0x10001]>;

/// グラフの各頂点の入次数を表す配列。
type InDegs = Box<[usize; 0x10001]>;

/// 制御フローグラフの逆グラフ T, およびその入次数配列を作る。
fn make_transpose_graph(analysis: &Analysis, input: &Input) -> (Graph, InDegs) {
    const VERTEX_C: usize = 0x10000;

    let mut graph_t: Graph = vec![vec![]; 0x10001].try_into().unwrap();
    let mut in_degs: InDegs = vec![0; 0x10001].try_into().unwrap();

    // 辺 src -> dst を逆向きにした辺を追加する。
    macro_rules! add_edge {
        ($src:expr, $dst:expr) => {{
            graph_t[$dst].push($src);
            in_degs[$src] += 1;
        }};
    }

    for src in Address::all() {
        match analysis[src] {
            AnalysisKind::Unknown => {
                if let Some(succ_addrs) = get_succ_addrs(input, src) {
                    for succ_addr in succ_addrs {
                        match succ_addr {
                            SuccAddr::Somewhere => add_edge!(usize::from(src), VERTEX_C),
                            SuccAddr::Addr(dst) => add_edge!(usize::from(src), usize::from(dst)),
                        }
                    }
                } else {
                    add_edge!(usize::from(src), usize::from(src));
                }
            }
            AnalysisKind::Code => add_edge!(usize::from(src), usize::from(src)),
            AnalysisKind::NotCode => {}
        }
    }

    // 頂点 C に自己ループを持たせる。
    add_edge!(VERTEX_C, VERTEX_C);

    (graph_t, in_degs)
}

/// `Code` から一意に辿れる制御フローを `Code` とする。
fn analyze_code(analysis: &mut Analysis, input: &Input) {
    let mut visited: Box<[bool; 0x10000]> = vec![false; 0x10000].try_into().unwrap();

    macro_rules! visit {
        ($addr:expr) => {{
            if visited[usize::from($addr)] {
                false
            } else {
                visited[usize::from($addr)] = true;
                true
            }
        }};
    }

    for addr in Address::all() {
        if !visit!(addr) {
            continue;
        }
        if analysis[addr] != AnalysisKind::Code {
            continue;
        }

        let mut src = addr;
        while let Some(dst) = get_unique_succ_addr(analysis, input, src) {
            if !visit!(dst) {
                break;
            }
            analysis[dst] = AnalysisKind::Code;
            src = dst;
        }
    }
}

/// `Code` であるアドレス `addr` から一意に辿れる具体的に確定可能な後続アドレスがあればそれを返す。
fn get_unique_succ_addr(analysis: &Analysis, input: &Input, addr: Address) -> Option<Address> {
    // get_succ_addrs() の結果が具体的に確定可能なアドレスのみであり、
    // かつ NotCode なアドレスを除いた結果がちょうど 1 つだけならそれを返す。

    assert_eq!(analysis[addr], AnalysisKind::Code);

    let addrs: ArrayVec<Address, 2> = get_succ_addrs(input, addr)?
        .into_iter()
        .map(|succ_addr| match succ_addr {
            SuccAddr::Somewhere => None,
            SuccAddr::Addr(dst) => Some(dst),
        })
        .collect::<Option<_>>()?;

    let addrs: ArrayVec<Address, 2> = addrs
        .into_iter()
        .filter(|&addr| analysis[addr] != AnalysisKind::NotCode)
        .collect();

    (addrs.len() == 1).then(|| addrs[0])
}

/// あるアドレス上の命令から生じる 1 つの後続アドレス。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SuccAddr {
    /// 具体的に確定できない場合。
    Somewhere,
    /// 具体的に確定できる場合。
    Addr(Address),
}

type SuccAddrs = ArrayVec<SuccAddr, 2>;

/// 指定したアドレス上の命令を取得し、そこから生じる全ての後続アドレス(最大 2 つ、重複なし)を返す。
/// 命令が取得できない場合、`None` を返す。
fn get_succ_addrs(input: &Input, addr: Address) -> Option<SuccAddrs> {
    let memory = input.memory();
    let (op, bank_id) = memory.fetch_op(addr).ok()?;

    let determine = |dst: Address| {
        // 命令とその後続アドレスがともに同一バンク上にあるか、
        // 後続アドレスが非固定バンク上になければ確定可能。
        let ok = memory.find_bank_id(dst).map_or(true, |dst_bank_id| {
            bank_id == dst_bank_id || !memory.banks()[dst_bank_id].is_fixed()
        });
        if ok {
            SuccAddr::Addr(dst)
        } else {
            SuccAddr::Somewhere
        }
    };

    let mut res = SuccAddrs::new();

    // ここでは行き先が別バンクになるケースも許す。
    // (そのようなケースのうち Unknown なものについては命令単位の解析で排除済み)
    match memory.resolve_op_succ(addr, op.succ()) {
        OpSuccResolved::Normal(dst) => res.push(determine(dst)),
        OpSuccResolved::Brk(dst) => res.push(dst.map_or(SuccAddr::Somewhere, determine)),
        OpSuccResolved::Kil => res.push(SuccAddr::Somewhere),
        OpSuccResolved::Branch { taken, not_taken } => {
            // 分岐時/非分岐時の行き先が同じケースがあることに注意。
            let taken = determine(taken);
            let not_taken = determine(not_taken);
            res.push(taken);
            if taken != not_taken {
                res.push(not_taken);
            }
        }
        OpSuccResolved::Jsr(dst) => res.push(determine(dst)),
        OpSuccResolved::Rti => res.push(SuccAddr::Somewhere),
        OpSuccResolved::Rts => res.push(SuccAddr::Somewhere),
        OpSuccResolved::JmpAbs(dst) => res.push(determine(dst)),
        OpSuccResolved::JmpInd(dst) => res.push(dst.map_or(SuccAddr::Somewhere, determine)),
    }

    Some(res)
}
