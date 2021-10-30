// Copyright 2021 AuthForbidden
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{cmp::Reverse, collections::HashMap, fs::File, io::{self, BufRead}, path::Path};

fn generate_dict() {
    let mut poem_stat: HashMap<char, u32> = HashMap::new();
    if let Ok(poem_lines) = read("poems.txt") {
        for poem_rline in poem_lines {
            if let Ok(poem_line) = poem_rline {
                if poem_line.starts_with('(') || poem_line.contains('【') {
                    continue;
                }
                for poem_char in poem_line.chars() {
                    if poem_char != '，' && poem_char != '。' && poem_char != '-' {
                        *poem_stat.entry(poem_char).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    let mut poem_items: Vec<_> = poem_stat.iter().collect();
    poem_items.sort_by_key(|item| Reverse(item.1));
    let poem_keys = poem_items.iter().map(|item| *item.0).collect::<String>();
    println!("{:?}", poem_keys);
}

fn read<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P : AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
