#!/usr/bin/env python3
import sys
from pathlib import Path
from collections import deque, defaultdict

EXAMPLE: bool = False

if EXAMPLE:
    INPUT_PATH = Path("inputs/25.ex")
    SPLITS = [("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")]
else:
    INPUT_PATH = Path("inputs/25.in")
    SPLITS = [("bdj", "vfh"), ("ztc", "ttv"), ("bnv", "rpd")]

INPUT_ADJ = {
    parts[0].rstrip(":"): parts[1:]
    for parts in (
        line.split() for line in INPUT_PATH.read_text().splitlines() if line
    )
}


def make_edge(a: str, b: str) -> tuple[str, ...]:
    return tuple(sorted([a, b]))


def solve(graph: bool = False):
    out = "digraph G { \n edge [dir=none, color=red];\n "
    adj = defaultdict(list)
    seen = set()
    splits = {make_edge(a, b) for (a, b) in SPLITS}
    for k, vs in INPUT_ADJ.items():
        for v in vs:
            edge = make_edge(k, v)
            if edge not in seen and (graph or edge not in splits):
                adj[k].append(v)
                adj[v].append(k)
                seen.add(edge)
                out += f'"{k}" -> "{v}" [label="{k}:{v}"];\n'
    out += "}\n"
    if graph:
        print(out)
    else:
        edge = splits.pop()
        print(bfs(edge[0], adj) * bfs(edge[1], adj))


def bfs(start, adj):
    q = deque([start])
    seen = set()
    while len(q) != 0:
        cur = q.popleft()
        if cur in seen:
            continue
        seen.add(cur)
        q.extend(adj.get(cur, []))
    return len(seen)


if __name__ == "__main__":
    """
    Identify splits visually by running with "--graph",
    then capture them in the SPLITS constant at the top of the script:
    
    ```
    ./25.py --graph | dot -Tsvg -o out.svg
    ```
    """
    solve("--graph" in " ".join(sys.argv[1:]))
