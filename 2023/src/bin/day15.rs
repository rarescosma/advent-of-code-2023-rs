use aoc_prelude::HashSet;

#[derive(Debug)]
enum InstrKind {
    Assign(u8),
    Remove,
}

#[derive(Debug)]
struct Instr<'a> {
    label: &'a str,
    kind: InstrKind,
    hash: u32,
}

impl From<&'static str> for Instr<'_> {
    fn from(s: &str) -> Instr {
        let (hash, label, kind) = if s.contains('-') {
            let label = s.split('-').next().expect("called contains");
            (hash(label), label, InstrKind::Remove)
        } else {
            let mut words = s.split('=');
            let label = words.next().unwrap();
            let focal = words.next().unwrap().parse::<u8>().unwrap();
            (hash(label), label, InstrKind::Assign(focal))
        };
        Instr { label, kind, hash }
    }
}

fn hash(s: &str) -> u32 {
    let mut ans = 0;
    let mask = (1 << 8) - 1;

    for c in s.chars() {
        ans += c as u32;
        // mul by 17
        let c = ans;
        ans <<= 4;
        ans += c;

        // mask
        ans &= mask;
    }

    ans
}

#[derive(Default, Debug)]
struct TheBox {
    known_labels: HashSet<String>,
    lenses: Vec<(String, u8)>,
}

fn solve() -> (u32, usize) {
    let mut boxes = Vec::with_capacity(256);
    boxes.extend((0..256).map(|_| TheBox::default()));

    let input = include_str!("../../inputs/15.in").trim().split(',');

    let p1 = input.clone().map(hash).sum::<u32>();

    input.map(Instr::from).for_each(|i| {
        let the_box = &mut boxes[i.hash as usize];
        let label = i.label.to_owned();
        match i.kind {
            InstrKind::Remove => {
                if the_box.known_labels.contains(&label) {
                    the_box.known_labels.remove(&label);
                    the_box.lenses.retain(|(l, _)| l != &label);
                }
            }
            InstrKind::Assign(focal) => {
                if the_box.known_labels.contains(&label) {
                    // replace
                    let pos = the_box
                        .lenses
                        .iter()
                        .position(|(l, _)| l == &label)
                        .unwrap();
                    the_box.lenses[pos] = (label.clone(), focal);
                } else {
                    // insert
                    the_box.known_labels.insert(label.clone());
                    the_box.lenses.push((label, focal));
                }
            }
        }
    });

    let p2 = boxes
        .into_iter()
        .enumerate()
        .flat_map(move |(b_no, b)| {
            b.lenses
                .into_iter()
                .enumerate()
                .map(move |(s_no, lens)| (1 + b_no) * (1 + s_no) * lens.1 as usize)
        })
        .sum::<usize>();

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
