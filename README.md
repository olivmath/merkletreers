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

```toml
#Cargo.toml

[dependencies]
merkletreers = "0.4.0"
```

## How to works

- *We use keccak-256 under-the-hood*

This library provides a clean and easy to use implementation of the Merkle Tree with the following features:

- Create Leaf
- Create Root
- Create Proof
- Verify Proof


![](/asset.png)

## How to Use

**Create a Merkle Tree**

```rust
use merkletreers::merkletree::tree::MerkleTree;

let tree = MerkleTree::new(vec!["a","b","c","d"]);

assert_eq!(tree.leafs(), [
    "3ac225168df54212a25c1c01fd35bebfea408fdac2e31ddd6f80a4bbf9a5f1cb",
    "b5553de315e0edf504d9150af82dafa5c4667fa618ed0a6f19c69b41166c5510",
    "0b42b6393c1f53060fe3ddbfcd7aadcca894465a5a438f69c87d790b2299b9b2",
    "f1918e8562236eb17adc8502332f4c9c82bc14e19bfc0aa10ab674ff75b3d2f3",
])
```


**Create a Root**

```rust
use merkletreers::merkletree::tree::MerkleTree;

let tree = MerkleTree::new(vec!["a","b","c","d"]);

assert_eq!(
    tree.root(),
    vec!["115cbb4775ed495f3d954dfa47164359a97762b40059d9502895def16eed609c"],
);
```

**Create Proof of a leaf**
```rust
use merkletreers::merkletree::{tree::MerkleTree, node::Node};

let mtree = MerkleTree::new(vec!["a", "b", "c", "d"]);
assert_eq!(
    mtree.proof("a"),
    vec![
        Node::left("3ac225168df54212a25c1c01fd35bebfea408fdac2e31ddd6f80a4bbf9a5f1cb"),
        Node::right("b5553de315e0edf504d9150af82dafa5c4667fa618ed0a6f19c69b41166c5510"),
        Node::right("64673cf40035df6d3a0d0143cc8426de49b9a93b9ad2d330cb4f0bc390a86d20")
    ]
);
```

**Verify Proof of a leaf**
```rust
```


## Roadmap

| Feature | Status | Priority |
|-|-|-|
| Create Root | ✅ | 🔥 |
| Create Proof | ✅ | 🔥 |
| Verify Proof | ⏰ | 🔥 |
| Support **[OpenZeppelin](https://docs.openzeppelin.com/contracts/4.x/utilities#verifying_merkle_proofs)** | ⏰ | 🔥 |
| Compatible with **[MerkleTreeJs](https://github.com/miguelmota/merkletreejs)** | ⏰ | 🔥 |
| Use any Hash function | ⏰ | 🧐 |
| Leafs of any size | ⏰ | 🧐 |

## Contributing

- Before read a code of conduct: **[CODE_OF_CONDUCT](CODE_OF_CONDUCT.md)**
- Follow the guide of development: **[CONTRIBUTING](CONTRIBUTING.md)**

## License

[MIT](LICENSE)
