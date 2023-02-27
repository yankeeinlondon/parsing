# Markdown

> [Commonmark Spec](https://spec.commonmark.org/0.30/)

## High Level Structure

Before creating the parser, it sometimes helps to think in pictures of high-level relationships:

```mermaid
flowchart LR

    Page(Page) -->|composed of| Blocks

    subgraph blocks
    Blocks -.-> Quote([Block Quote])
    Blocks -.-> Code([Code Block])
    Blocks -.-> FencedBlock([Fenced Block])
    Blocks -.-> List([List])
    Quote -.-> B2I
    Code -.-> B2I
    List -.-> B2I
    FencedBlock -.-> B2I
    end

    B2I(( ))

    subgraph inline
    B2I ==>|composed of| Inline(Inline Text)

    Inline ==> S([&nbsp;])
    S --> heading
    S --> italic
    S --> bold
    S --> link
    S --> line

    end
```

This diagram is not meant to capture all of the rules/tokens/meaning found in a Markdown document but enough to help show the high-level relationships that a parser should be looking to reinforce.

Ultimately a parsed output of a Markdown page should resemble a tree structure something like the following:

```mermaid
mindmap
    Document
        b1[block]
            i1[inline]
            i2[inline]
            i3[inline]
            i4[inline]
            i5[inline]
        b2[block]
            i6[inline]
            i7[inline]
        b3[block]
            i8[inline]
            i9[inline]
            i10[inline]
```

This should give us a good start in our strategy for building out the parser.
