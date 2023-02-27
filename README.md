# Parsing Project

A simple test of three parsing crates in Rust. The primary objective is to simply understand the way these three popular crates work:

- What is the **learning curve** involved?
  - How good is the documentation?
  - Are there good examples which can be leveraged?
  - How clear is the API surface?
- How **performant** can we expect these parsers to be?

> **Note:** this is _not_ meant to be a comparison of the three so you can choose which to use and which to disregard. They all target different problem scopes. Sorry but "it depends" is still the dominant answer in the universe.

## Project Approach

For all three tests we'll explore parsing out the following features in a Markdown file.

> All reference definitions of Markdown should come from the [Commonmark Spec](https://spec.commonmark.org/0.30/).

### Base Markdown Features

- `blocks` at the top level a Markdown file is composed of _blocks_ of content
  - We can also sub-type blocks in `leaf-blocks` and `container-blocks`
- `inline` inline content is _contained_ inside blocks and represents the majority of the "prose" in any given document.

Within the `blocks` we may also want to call out specifically the following types of block elements:

- `list_item` - a single list item is a block
- `list` - a grouping of list items
- `blockquote` - a block element that is contained in another block
- `code_block`

For the `inline` sections we will define:

- `headings` pull out headers (their level and the text)
- `links` pull out the text and URI information of a Markdown link
- We'll also cover the most basic styling elements:
  - `italic` as text surrounded by \_ characters
  - `bold` as text surrounded by \*\* characters

### Extending Markdown

To test our abilities to go a little outside the Markdown language specification:

- `multi-word` we'll build up the ability to parse multi-word descriptors that are expressed in `kebab-case`, `camelCase`, `PascalCase`, and `snake_case`.
- `emoji`  we'll add a feature to all three parsers where any inline text which is surrounded by `::` markers will be evaluated as a possible emoji character.
- `emoji-plus` while the above language token is an _inline_ element we'll also distinguish an emoji which is defined as a "block element":
  - The key distinguishing features for the _block_ element will be two fold:
    1. **Size** - the block element will be much larger than the inline element
    2. **Styling** - you can optionally add HTML attributes to your emoji.
  - The token will have to start a line with `::`
  - then any alphabetic character will be used to match to known emoji's (same as regular emoji)
  - If an optional opening and closing curly brace is detected then we'll parse that as a `dictionary` of key/value pairs
  - Any key/value pairs found in a `dictionary` will be translated to _attributes_ on the emoji's surrounding `<span/>` tag.

#### Supported Emoji's

- For now we'll just support the following:
  - 😀 smile
  - 😢 cry
  - 👍 thumbs_up / thumbs-up / thumbsUp / ThumbsUp / thumbs up
  - 👎 thumbs_down / thumbs-down

### Why Markdown?

This is really just an exercise in getting familiar with different parsers available to the Rust syntax and using **Markdown** as the "target" language is just for familiarity purposes.

Furthermore, since both Nom and Pest are _generalized_ parsers they can adapt to any problem surface and in the "obvious utility" realm I think MD-to-HTML parsing is a problem that many people are both familiar with (and likely benefiting from in some way).

> Note: if you really need to parse Markdown you'd almost surely want to use a proper Markdown parser like `pulldown-cmark` which has been thoroughly tested and is custom built to be performant for the task at hand.

## The Contenders

1. **Pest**([github](https://github.com/pest-parser/pest), [api docs](https://docs.rs/nom/latest/nom/), [site](https://pest.rs), [book](https://pest.rs/book/))

    A well implemented and documented crate that allows parsers to be setup via using a single [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) grammar file.

    ```rust
    extern crate pest;
    #[macro_use]
    extern crate pest_derive;
    use pest::Parser;

    #[derive(Parser)]
    #[grammar = "my-language.pest"]
    struct MyLanguage;

    fn main() {
        let tokens = MyLanguage::parse(Rule::markdown, "# Hello World")
            .unwrap_or_else(|e| panic!("{}", e));
        // ...
    }
    ```

    PEG grammar files were formalized in 2004. This makes them relatively new in system design parlance but mature enough to be battle tested and refined enough to be considered production ready. PEG's are intended for building non-ambiguous (read: idempotent) parsing trees for a given grammar but their problem space remains largely for _computer languages_ more so than _spoken languages_.

    One example of this [CPython](https://en.wikipedia.org/wiki/CPython) which is a modern parser replacement to the original

2. **NOM**([nom](https://docs.rs/nom/latest/nom/), [nom supreme](https://docs.rs/nom-supreme/latest/nom_supreme/))

    NOM is a well respected generalized parsing crate in the Rust ecosystem. Unlike Pest, it doesn't use grammars but instead leverages parser functions which act as _combinators_ and can be chained together for significant reuse. This crate may not be able to beat a bespoke parser in performance but they do generate statically linked general parsers which are quite fast in their category and using a crate like this should drastically increase your ability to parse a DSL or any non-standard input.

3. [**Pulldown CMark**](https://github.com/raphlinus/pulldown-cmark)

    Unlike the prior two crates, `pulldown-cmark` is a _specific_ parser for Markdown content. And of the crates I've seen in Rust, it is almost surely the fastest. In addition it is fully [Commonmark](https://commonmark.org/) compliant.

## One Step, Two Step

Our ultimate goal in this repo is to transform Markdown content (good for writing) into HTML content (good for presentation).

Now while parsing is the act of taking an unintelligible stream/file/source of data and _parsing_ it into tokens so that we can gain meaning from the data ... that meaning will not be expressed in HTML. So in fact we have two steps to take:

1. Parse (make sense of the Markdown content)
2. Transform (transform our understanding into HTML)

## Command Line

All three monorepos will leverage the superb [clap](https://github.com/clap-rs/clap) crate for CLI actions and all three will share the same CLI API:

```sh
# parse the markdown file and export the tokenized output to stdout
parse [markdown file]
# parse the markdown file and convert to HTML
parse --html [markdown file]
```

> yeah I know ... "fancy", eh?

## Tests

Tests are broken up into two types:

1. **units** - used while I was experimenting and learning the parsers to validate that I was getting the results I expected.
2. **performance** - while this is not intended in any way to be comprehensive or _smart_ performance tests the [criterion](https://crates.io/crates/criterion) crate does make it easy to make at least "ok" tests to get some ballpark comparisons.

## Conclusions

Conclusions vary by user so as to not influence yours, my opinions have been added to the [conclusions](conclusions.md) markdown file in the root of this repo.

If you couldn't care less about my conclusions but want an outline of my notes _per parser_ then:

- [pest](docs/pest.md)
- [nom](docs/nom.md)
- [pulldown-cmark](docs/pulldown-cmark.md)

## Contributing

I'm 100% open to any Issue or PR relating to documentation or code. Just make sure that with PR's:

- all tests pass (adding tests if this is appropriate) for code changes
- all clippy suggestions/styles implemented
- ideally a spell checker on any documentation code changes (I use the **cSpell** plugin with vs-code and you can feel free to add language exceptions at `~/.vscode/settings.json` so long as it's not a "made up word") ... zero requirement to use this plugin however.

> Note: i have my hands in a lot of baskets so I can often miss new issues or PRs. If it's been more than a week on a PR and you feel you need to reach out then ping me on Discord at `YankeeInLondon#9039` and I'll usually respond relatively soon (also give context in who you are and why you're pinging). If this is for an **Issue** please bear in mind that these are always given much lower priority than a PR so consider maybe upgrading to a PR if you've got the time.
