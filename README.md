# 🌳 Merkle Tree

The **simple and easy** implementation of **Rust Merkle Tree**

---

<!-- [![CodeQL](https://github.com/olivmath/merkly/actions/workflows/codeql-analysis.yml/badge.svg)](https://github.com/olivmath/merkly/actions/workflows/codeql-analysis.yml)
[![Lint](https://github.com/olivmath/merkly/actions/workflows/lint.yml/badge.svg)](https://github.com/olivmath/merkly/actions/workflows/lint.yml)
[![Test](https://github.com/olivmath/merkly/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/olivmath/merkly/actions/workflows/test.yml)
[![PyPI](https://img.shields.io/pypi/v/merkly)](https://pypi.org/project/merkly/)

![GitHub last commit](https://img.shields.io/github/last-commit/olivmath/merkly)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/olivmath/merkly)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/merkly)](https://pypi.org/project/merkly/)
[![PyPI - Downloads](https://img.shields.io/pypi/dm/merkly)](https://pypi.org/project/merkly/)
![PyPI - License](https://img.shields.io/pypi/l/merkly) -->

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


## How to works

- *We use keccak-256 under-the-hood*

This library provides a clean and easy to use implementation of the Merkle Tree with the following features:

- Create Leaf
- Create Root
- Create Proof
- Verify Proof

<img width="2216" alt="merkle-tree" src="https://user-images.githubusercontent.com/50037567/193623013-3c2888d1-441f-45a1-97fe-6e275edde847.png">


## How to Use

**Create a Merkle Tree**

```rust
use merkletreers::mtree::MerkleTree;

let tree = MerkleTree::new(vec![
    "a".to_string(),
    "b".to_string(),
    "c".to_string(),
    "d".to_string(),
]);

assert_eq!(tree.leafs(), [
    "3ac225168df54212a25c1c01fd35bebfea408fdac2e31ddd6f80a4bbf9a5f1cb".to_string(),
    "b5553de315e0edf504d9150af82dafa5c4667fa618ed0a6f19c69b41166c5510".to_string(),
    "0b42b6393c1f53060fe3ddbfcd7aadcca894465a5a438f69c87d790b2299b9b2".to_string(),
    "f1918e8562236eb17adc8502332f4c9c82bc14e19bfc0aa10ab674ff75b3d2f3".to_string()
])
```


**Create a Root**

```rs
```

**Create Proof of a leaf**
```rs
```

**Verify Proof of a leaf**
```rs
```


## Roadmap

| Feature | Status | Priority |
|-|-|-|
| Create Root | ⏰ | 🔥 |
| Create Proof | ⏰ | 🔥 |
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
