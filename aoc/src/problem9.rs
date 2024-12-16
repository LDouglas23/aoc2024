use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::{Display, Write},
};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Block {
    Free,
    File(usize),
}

type FileId = usize;

#[derive(Debug, Clone)]
struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    fn compact(&mut self) {
        let mut cursor_l = 0;
        let mut cursor_r = self.blocks.len() - 1;

        while cursor_l != cursor_r {
            if let Block::File(_) = self.blocks[cursor_r] {
                if let Block::Free = self.blocks[cursor_l] {
                    self.blocks.swap(cursor_l, cursor_r);
                    cursor_r -= 1
                } else {
                    cursor_l += 1;
                }
            } else {
                cursor_r -= 1;
            }
        }
    }

    fn compact_chunks(&mut self) {
        let mut chunk_map = ChunkMap::from(self.borrow());

        let max_file_id = chunk_map.files.keys().max().unwrap();

        for id in (0..*max_file_id + 1).rev() {
            let file_span = chunk_map.files.get(&id).unwrap();

            if let Some(free_span) = chunk_map
                .free
                .iter()
                .find(|&span| span.len() >= file_span.len())
            {
                let mut j = free_span.start_index;

                if free_span.start_index < file_span.start_index {
                    for i in file_span.start_index..file_span.end_index {
                        self.blocks.swap(j, i);

                        j += 1;
                    }
                }
            }

            chunk_map = ChunkMap::from(self.borrow());
        }
    }

    fn checksum(&self) -> usize {
        let mut result = 0;
        let mut iter = self.blocks.iter().enumerate();

        while let Some((i, block)) = iter.next() {
            if let Block::File(id) = block {
                result += i * id;
            }
        }

        result
    }
}

impl From<String> for Disk {
    fn from(value: String) -> Self {
        let mut file_id = 0;
        let mut blocks: Vec<Block> = vec![];

        for (i, c) in value.chars().enumerate() {
            let num_blocks = c.to_digit(10).expect("failed to convert char to digit");

            if i % 2 == 0 {
                blocks.append(&mut (0..num_blocks).map(|_| Block::File(file_id)).collect());
                file_id += 1;
            } else {
                blocks.append(&mut (0..num_blocks).map(|_| Block::Free).collect());
            }
        }

        Self { blocks }
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            match block {
                Block::File(id) => f.write_str(&id.to_string())?,
                Block::Free => f.write_char('.')?,
            };
        }

        Ok(())
    }
}

pub struct Input {
    map: String,
}

impl From<Vec<String>> for Input {
    fn from(value: Vec<String>) -> Self {
        Self {
            map: value[0].to_owned(),
        }
    }
}

pub fn solution(input: Input) -> usize {
    let mut disk = Disk::from(input.map);
    disk.compact();

    disk.checksum()
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Span {
    start_index: usize,
    end_index: usize,
}

impl Span {
    fn len(&self) -> usize {
        self.end_index - self.start_index
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ChunkMap {
    free: Vec<Span>,
    files: HashMap<FileId, Span>,
}

impl From<&Disk> for ChunkMap {
    fn from(value: &Disk) -> Self {
        let mut free = vec![];
        let mut files = HashMap::new();
        let mut index = 0;

        for chunk in value.blocks.chunk_by(|a, b| a == b) {
            let span = Span {
                start_index: index,
                end_index: index + chunk.len(),
            };

            match chunk[0] {
                Block::File(id) => {
                    files.insert(id, span);
                }
                Block::Free => {
                    free.push(span);
                }
            }

            index += chunk.len()
        }

        Self { free, files }
    }
}

pub fn solution_part_two(input: Input) -> usize {
    let mut disk = Disk::from(input.map);

    disk.compact_chunks();

    disk.checksum()
}

#[cfg(test)]
mod test {
    use crate::problem9::solution_part_two;

    use super::{solution, Input};

    #[test]
    pub fn test() {
        let map = "2333133121414131402";

        assert_eq!(
            solution(Input {
                map: map.to_owned()
            }),
            1928
        )
    }

    #[test]
    pub fn test2() {
        let map = "2333133121414131402";

        assert_eq!(
            solution_part_two(Input {
                map: map.to_owned()
            }),
            2858
        );
    }
}
