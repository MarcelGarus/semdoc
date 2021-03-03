This document describes what a SemDoc file contains.
We'll do so by lowering the following "Hello, world!" document:

> ### SemDoc
>
> Hello, world!
>
> This is a test.

Btw: What we'll do now is exactly what the *SemDoc engine* does when saving a SemDoc – you might want to check out the Rust code that does this; it's available as the `semdoc` crate.

You can also inspect each of these layers of a SemDoc by using the `semdoc file.sd inspect <layer name>` command.

### Blocks

A SemDoc is composed of *blocks*.
Each block is a little piece of the document.
Blocks are standalone and self-contained.

There exist many kinds of blocks:

* A **Text** contains textual information.
* A **Section** contains two other blocks: a title and content.
* A **SplitSequence** contains multiple sequential other blocks with a little information gap in between.
* Many more blocks exist. (TODO: Add link.)

The "Hello, world!" document will be represented as the following tree of blocks:

* **Section**
  * title: **Text**: `"SemDoc"`
  * body: **SplitSequence**
    * **Text**: `"Hello, world!"`
    * **Text**: `"This is a test."`

### Molecules

Next, blocks are lowered into *molecules*.
There are only two types of molecules: bytes and blocks (containing a kind and possibly multiple children).
So, after this step, the huge variety of blocks got reduced to a tree containing only these two primitives.

For example, an image might be turned into a block molecule containing three byte molecules for the aspect ratio, blur hash and actual image content.

Our "Hello, world!" document might be lowered into this molecule tree (bytes are in hex):

* **Block**, kind 2 (Section)
  * **Block**, kind 1 (Text)
    * **Bytes**: `53 65 6d 44 6f 63`
  * **Block**, kind 4 (SplitSequence)
    * **Block**, kind 1 (Text)
      * **Bytes**: `48 65 6c 6c 6f 2c 20 77 6f 72 6c 64 21`
    * **Block**, kind 1 (Text)
      * **Bytes**: `54 68 69 73 20 69 73 20 61 20 74 65 73 74 2e 20 48 65 6c 6c 6f 21`

### Atoms

Next, the molecules are lowered into *atoms*.
Until now, the document has been represented as a tree.
Atoms are a serialized version of the tree – they are a depth first traversal of the tree.

There are several more types of atoms than molecules:

* **Block**: An atom containing a kind, followed by its children.
* **SmallBlock**: Same as the Block atom, but it can only save 255 children. It's encoded more efficiently later on.
* **Bytes**: An atom containing bytes.
* **FewBytes**: Same as the Bytes atom, but it can only save 255 bytes. It's encoded more efficiently later on.
* **Reference**: An atom that can point to an atom saved somewhere after it in the file. This is very handy for large subtrees like the bytes of an image. On SemDocs with random access, this enables deserializing multiple blocks simultaneously using multiple threads. When loading SemDocs sequentially (like, when downloading them), it enables loading text first and images later.

The "Hello, world" document from above could be converted into these atoms:

* SmallBlock, kind 2, 2 children
* SmallBlock, kind 1, 1 child
* FewBytes, 6 bytes long: `53 65 6d 44 6f 63`
* SmallBlock, kind 4, 2 children
* SmallBlock, kind 1, 1 child
* FewBytes, 13 bytes long: `48 65 6c 6c 6f 2c 20 77 6f 72 6c 64 21`
* SmallBlock, kind 1, 1 child
* FewBytes, 22 bytes long: `54 68 69 73 20 69 73 20 61 20 74 65 73 74 2e 20 48 65 6c 6c 6f 21`

### Bytes

The atoms are then lowered into bytes, aligned to 8 bytes.

Explaining the structure here would be a bit clumsy, so I'll refer to the `semdoc helloworld.sd inspect bytes` command, which displays the bytes along with further information.
It also colors the bytes so the format is pretty self-explanatory.

Just for the record, here's the encoding of the document above:

```
53 65 6d 44 6f 63 00 00
00 02 00 00 00 00 00 02
00 01 00 00 00 00 00 01
03 06 53 65 6d 44 6f 63
00 02 00 00 00 00 00 04
00 01 00 00 00 00 00 01
03 0d 48 65 6c 6c 6f 2c
20 77 6f 72 6c 64 21 00
00 01 00 00 00 00 00 01
03 16 54 68 69 73 20 69
73 20 61 20 74 65 73 74
2e 20 48 65 6c 6c 6f 21
```
