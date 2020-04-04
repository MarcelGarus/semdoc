RDF enables everyone to create distribution-ready documents.

If most of the control is shifted towards the writer, she can exactly define how the resulting document looks like.
PDF uses this approach: There's 

The control of how the document looks

Most popular document formats 

## Goals

- **portable:** RDFs should be readable on every platform.
- **high-level:** Writers should concern themselves about what a document contains rather than how it looks.
- **adaptive:** RDFs adapt to the available screen size, and the reader's preferred font, text size, brightness etc.
- **modular:** Features should be able to be added.
- **synchronization-ready:** Documents get shared. It should be possible to merge documents by different users and devices.
- **support long documents:** Writing books in RDF should be possible.

RDF is *not*

- *pixel-perfect:* Use PDF if you need fine-grained control about how your document looks.
- *intended to be edited by hand:* Rich editors exist for a reason. Use them.
- *highly interactive*: Create an application or a website if you need that.

## Why is it needed?

There's a fundamental tradeoff in the design of how document formats: 
If writers are able to determine font sizes, colors, and layout, the reader loses control over these properties.

PDF gives the writers most of the control.
As a result, writers can implement all kinds of media — diagrams, music notes, comics — that were not anticipated by the PDF format.
On the other hand, the reader is presented with a pretty inflexible set of pages that don't adapt to fonts, screen and text sizes, dark mode and other configuration options.

Markdown leaves the writers only with very few primitives, giving readers most of the control.
Dark theme, reading on mobile etc. all work very well and are reasons why Markdown is widely adopted recently.
On the other hand, it's often difficult to express something more elaborate in Markdown:
Implementing diagrams, music notes and other kinds of media is quite hard to do.
That's why people resort to workarounds like ascii art or svgs to create diagrams.
They don't look as good as they could be and they don't always work well with dark mode or smaller screens.



RDF aims to be extensible so these can be primitives.

If the writer controls how the document is formatted, it's possible to represent anything with that document.
An example is PDF:
Because PDF writers can control exactly how the final document will look like, they can easily implement diagrams, music notes, and lots of other media that the PDF format didn't anticipate.  
On the other hand, if the reader controls how the document is formatted, it's possible to adapt the format to special needs.
For example, in Markdown, the reader can enable dark mode, choose different fonts, increase font sizes etc.

There's a direct tradeoff of control between the writer and the reader:



