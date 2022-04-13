# Wharf

A language agnostic code documentation utility.

## Documentation
For detailed documentation, please see the [specifications](./spec.md).

### Supported Languages
Please see the [support](/support.md) table

### Whats it about?

Wharf is a code documentation utility which aims to provide best-in-class documentation across as many languages as
possible, through the use of regular code comments.

### How does it work?

Wharf scans your working directory, taking care not to violate your `.wharfignore` paths so that it can determine which
files to process, after the initial phase of processing, the program processes the files for comments in your code. When
it finds a comment, it checks if the comment matches the supported format, if it does, the program will attempt to
convert your comments into beautiful documentation based on the tags and descriptors you provide.

## Benefits

Why learn a different syntax for every language you use, when you can learn one syntax, and rely upon it no matter what
language you are coding with? Wharf makes it simple to do just that with its powerful documentation syntax.

### How do I use it?

You simply navigate to the root directory of the project you wish to document, and run `wharf`, once Wharf initializes
then you will be greeted by our helpful text interface that tells you how your documentation is being handled.

### How do I view my documentation?

Wharf compiles your documentation into HTML format, making it easy to view on any modern device using a web browser, and
enabling easy documentation publishing to the web.

## Similar Projects:

| Title                                                                                | Supported Languages      | Type        |
|:-------------------------------------------------------------------------------------|:-------------------------|:------------|
| [Cargo Doc](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)                 | Rust                     | Built In    |
| [JSDoc](https://github.com/jsdoc/jsdoc)                                              | JavaScript, (TypeScript) | Third Party |
| [ESDoc](https://esdoc.org/)                                                          | Javascript, (TypeScript) | Third Party |
| [PyDoc](https://docs.python.org/3/library/pydoc.html)                                | Python                   | Third Party |
| [DocFX](https://dotnet.github.io/docfx/tutorial/docfx_getting_started.html)          | C#                       | Third Party |
| [VSDocman](https://marketplace.visualstudio.com/items?itemName=PeterMacej.VSdocman)  | C#, Visual Basic         | Third Party |

There is a common trend amongst these projects, they tend to only focus on one language, and they all have their own syntax for powering their documentation.

The aim of this project is to support all of the languages listed above, and more, in a language agnostic manner that provides the same set of basic syntax, but is supported by a much wider array of languages.

We also wish to make the project extensible using plugins, that way the community can create features I could never dream of.

# Installation
> Coming soon....


## License
This project is licensed under the Mozilla Public License Version 2.0. The terms of this license can be found within [LICENSE.txt](LICENSE.txt)