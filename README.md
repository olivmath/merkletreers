# 🌳 Merkle Tree

The **simple and easy** implementation of **Rust Merkle Tree**

---

[![Build](https://github.com/olivmath/merkletreers/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/olivmath/merkletreers/actions/workflows/build.yml) [![Test](https://github.com/olivmath/merkletreers/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/olivmath/merkletreers/actions/workflows/test.yml) [![Crates.io](https://img.shields.io/crates/v/merkletreers.svg)](https://crates.io/crates/merkletreers)

![GitHub last commit](https://img.shields.io/github/last-commit/olivmath/merkletreers)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/olivmath/merkletreers)

![Crates.io](https://img.shields.io/crates/l/merkletreers)

## Table of Contents

- [Credits](#credits)
- [How to install](#how-to-install)
- [How it works](#how-it-works)
- [How to use](#how-to-use)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

## Credits

[![GitHub Contributors Image](https://contrib.rocks/image?repo=olivmath/merkletreers)](https://github.com/olivmath/merkletreers/graphs/contributors)

## How to install

```shell
cargo add merkletreers
```

## How to works

- _We use keccak-256 under-the-hood_

This library provides a clean and easy to use implementation of the Merkle Tree with the following features:

- Create Leaf
- Create Root
- Create Proof
- Verify Proof

![](/asset.png)

## How to Use

**Create a Merkle Tree**

```rust
use merkletreers::tree::MerkleTree;
use merkletreers::utils::hash_it;

// Hash each element of leaves to convert into a [u8;32]
let leaves = ["a", "b", "c", "d", "e"]
    .iter()
    .map(|data| {
        let mut buffer = [0u8; 32];
        hash_it(data.as_bytes(), &mut buffer);
        buffer
    })
    .collect::<Vec<[u8; 32]>>();

// Create our Merkle tree
let tree = MerkleTree::new(leaves);
```

**Create a Root**

```rust
use merkletreers::tree::MerkleTree;
use merkletreers::utils::hash_it;

// Hash each element of leaves to convert into a [u8;32]
let leaves = ["a", "b", "c", "d", "e"]
    .iter()
    .map(|data| {
        let mut buffer = [0u8; 32];
        hash_it(data.as_bytes(), &mut buffer);
        buffer
    })
    .collect::<Vec<[u8; 32]>>();

// Create our Merkle tree
let tree = MerkleTree::new(leaves);

// Create our Merkle Root
let root = tree.root;
assert_eq!(
    root,
    [
        29, 208, 210, 166, 174, 70, 109, 102, 92, 178, 110, 26, 49, 240, 124, 87, 174, 93, 247,
        210, 188, 85, 156, 213, 130, 109, 65, 123, 233, 20, 26, 93
    ]
);
```

**Create Proof of a leaf**

```rust
use merkletreers::node::{Node, Side};
use merkletreers::tree::MerkleTree;
use merkletreers::utils::hash_it;

// Hash each element of leaves to convert into a [u8;32]
let leaves = ["a", "b", "c", "d", "e"]
    .iter()
    .map(|data| {
        let mut buffer = [0u8; 32];
        hash_it(data.as_bytes(), &mut buffer);
        buffer
    })
    .collect::<Vec<[u8; 32]>>();

// Create our Merkle tree
let tree = MerkleTree::new(leaves);

// Create our Merkle Root
let root = tree.root;
assert_eq!(
    root,
    [
        29, 208, 210, 166, 174, 70, 109, 102, 92, 178, 110, 26, 49, 240, 124, 87, 174, 93, 247,
        210, 188, 85, 156, 213, 130, 109, 65, 123, 233, 20, 26, 93
    ]
);

// Create your Merkle Proof for 'c' element
// First we need hash element to convert into a [u8; 32]
let mut leaf = [0u8; 32];
hash_it("c".as_bytes(), &mut leaf);
let proof = tree.make_proof(leaf);
assert_eq!(
    vec![
        Node {
            data: [
                241, 145, 142, 133, 98, 35, 110, 177, 122, 220, 133, 2, 51, 47, 76, 156, 130,
                188, 20, 225, 155, 252, 10, 161, 10, 182, 116, 255, 117, 179, 210, 243
            ],
            side: Side::RIGHT
        },
        Node {
            data: [
                128, 91, 33, 216, 70, 177, 137, 239, 174, 176, 55, 125, 107, 176, 210, 1, 179,
                135, 42, 54, 62, 96, 124, 37, 8, 143, 2, 91, 12, 106, 225, 248
            ],
            side: Side::LEFT
        },
        Node {
            data: [
                168, 152, 44, 137, 216, 9, 135, 251, 154, 81, 14, 37, 152, 30, 233, 23, 2, 6,
                190, 33, 175, 60, 142, 14, 179, 18, 239, 29, 51, 130, 231, 97
            ],
            side: Side::RIGHT
        }
    ],
    proof
);
```

**Verify Proof of a leaf**

```rust
use merkletreers::node::{Node, Side};
use merkletreers::tree::MerkleTree;
use merkletreers::utils::hash_it;


// Hash each element of leaves to convert into a [u8;32]
let leaves = ["a", "b", "c", "d", "e"]
    .iter()
    .map(|data| {
        let mut buffer = [0u8; 32];
        hash_it(data.as_bytes(), &mut buffer);
        buffer
    })
    .collect::<Vec<[u8; 32]>>();

// Create our Merkle tree
let tree = MerkleTree::new(leaves);

// Create our Merkle Root
let root = tree.root;
assert_eq!(
    root,
    [
        29, 208, 210, 166, 174, 70, 109, 102, 92, 178, 110, 26, 49, 240, 124, 87, 174, 93, 247,
        210, 188, 85, 156, 213, 130, 109, 65, 123, 233, 20, 26, 93
    ]
);

// Create your Merkle Proof for 'c' element
// First we need hash element to convert into a [u8; 32]
let mut leaf = [0u8; 32];
hash_it("c".as_bytes(), &mut leaf);
let proof = tree.make_proof(leaf);
assert_eq!(
    vec![
        Node {
            data: [
                241, 145, 142, 133, 98, 35, 110, 177, 122, 220, 133, 2, 51, 47, 76, 156, 130,
                188, 20, 225, 155, 252, 10, 161, 10, 182, 116, 255, 117, 179, 210, 243
            ],
            side: Side::RIGHT
        },
        Node {
            data: [
                128, 91, 33, 216, 70, 177, 137, 239, 174, 176, 55, 125, 107, 176, 210, 1, 179,
                135, 42, 54, 62, 96, 124, 37, 8, 143, 2, 91, 12, 106, 225, 248
            ],
            side: Side::LEFT
        },
        Node {
            data: [
                168, 152, 44, 137, 216, 9, 135, 251, 154, 81, 14, 37, 152, 30, 233, 23, 2, 6,
                190, 33, 175, 60, 142, 14, 179, 18, 239, 29, 51, 130, 231, 97
            ],
            side: Side::RIGHT
        }
    ],
    proof
);

// Verify our Merkle Proof for 'c' element
let result = tree.check_proof(proof, leaf);
assert_eq!(result, root);
```

## Important Notes

### Duplicate Leaves

The library handles duplicate leaves (same hash value at different positions) but with some limitations:

- ✅ **Root generation works correctly** - duplicates are treated as independent nodes
- ⚠️ **Proof generation always targets the FIRST occurrence** - cannot generate proofs for 2nd, 3rd, etc. occurrences
- ✅ **Proof verification works correctly** - generated proofs are mathematically valid

**Recommendation**: Avoid duplicate leaves when possible. If you need to include duplicate values, consider adding position/index information to make them unique:

```rust
// Instead of duplicates
let data = ["a", "b", "a", "c"];

// Make them unique by including position
let data_with_index = [
    ("a", 0),
    ("b", 1),
    ("a", 2),  // Now unique!
    ("c", 3),
];
```

See [ISSUE_11_FINDINGS.md](ISSUE_11_FINDINGS.md) for detailed investigation results.

## Roadmap

| Feature                                                                        | Status | Priority |
| ------------------------------------------------------------------------------ | ------ | -------- |
| Create Root                                                                    | ✅     | 🔥       |
| Create Proof                                                                   | ✅     | 🔥       |
| Verify Proof                                                                   | ✅     | 🔥       |
| Compatible with **[MerkleTreeJs](https://github.com/miguelmota/merkletreejs)** | ✅     | 🔥       |
| Compatible with **[Merkly](https://github.com/olivmath/merkly)**               | ✅     | 🔥       |
| Leafs of any size                                                              | ✅     | 🧐       |
| Use any Hash function                                                          | ⏰     | 🧐       |

## Contributing

- Before read a code of conduct: **[CODE_OF_CONDUCT](CODE_OF_CONDUCT.md)**
- Follow the guide of development: **[CONTRIBUTING](CONTRIBUTING.md)**

## License

[MIT](LICENSE)
