<img src="semdoc2.svg" />

## SemDoc – Semantic Document

SemDoc is a document file format that allows writers to focus on the content solely and empowers readers to adapt the appearance to their devices and needs.

### Why yet another file format?

There are many great tools for writing documents: Google Docs, Word, Notion, Markdown, Latex, and many more.
But for *distributing* documents, we're pretty much stuck with PDF.
SemDoc aims to change that.

**What's wrong with PDF?**
PDF is excellent for print-quality content:
If you want to print something or publish a design brochure, it's a great fit.
But I'm not too fond of PDF for general content consumption:

* **It organizes everything into pages.**
  It's not like scrolling is a particularly new invention.
* **The layout is entirely static.**
  I refuse to believe that putting my phone into landscape mode just to read a PDF without continually scrolling back and forth is the best user experience we can come up with in the 21st century.
* **Advanced features are hard to use correctly.**
  Tabbing through input fields or copying text with accents rarely works.
  These symptoms indicate an issue with the format itself.
* **Dark mode?** Anyone?

Fundamentally, PDF gives most of the control to document *creators*, leaving none to readers.

In constrast, SemDoc follows these principles:

* **Be a compile target.**
  It doesn't aim to be readable to humans or be edited by hand; instead, it's an efficient binary format.
  Like PDFs, you compile other formats to it.
* **Be purely semantic.**
  It contains no syntax information, only semantic information.
  Writers declare *what* to display, readers control *how* to show it.
* **Be extensible.**
  Over time, the SemDoc format can be extended.

<!--
I can almost hear people asking:
But what about just a tiny bit of syntax? I'm only asking for …

* **customizing the color scheme?**
  What if I told you some operating systems (for example, Windows) let the user choose a custom accent color?
  What about light and dark mode?
  What about blue mode? Red mode?
  What about AR glasses, where the world is the background?

* **overriding the default fonts?**
  Explain that to people with dyslexia, who use unique fonts that give each letter a recognizable characteristic.
  Debate with people who practiced speed reading with one particular font.
  And I can almost see the hacked-together "music fonts" and "math fonts" popping up for allowing you to distribute other content.
  Why bloat the document with font definitions and complicate everything?

* **make text bold?**
  You may mark content as *important*, but "bold" is such an arbitrary property.
  What about smart speakers reading the text out loud? Should they speak boldly?
  Why not give developers of AR glasses the freedom to mark important text by lifting it slightly to the front?
  Why artificially limit yourself to a concept that only makes sense on 2D screens?

Also, note that these little customization options all add up.
Over time, peer-pressure might build up that forces every writer to think of a fancy color scheme (of course, for light and dark mode), provide a font, etc.  
That's not what writers should have to concern themselves with.
If you do want more control, you're welcome to build a website.

Not defining the appearance also makes the format future-proof – it adapts to current devices just as well as AR glasses.
And a long-lived format is a win for everyone using it.
-->

### Relevance

Some might think, "documents are going to be cloud-first anyway. Consumers don't need file formats anymore; they'll collaborate online."
That's probably true for most cases.
But I'd argue there will always be a use-case for immutable atomic instances of documents to be sent.
Immutability makes them legally binding.
Latency might forbid you from collaborating with people living on Mars.
**The concept of transferring a file is straightforward to understand for us humans because files are like things.**

---

[Here's an explanation of the format.](format.md)

## Roadmap

- [x] Write vision
- [x] Implement base of the format
- [x] Make format debuggable
- [x] Document format
- [ ] Add more blocks
- [ ] Improve the quality of Markdown to SemDoc converter
- [ ] Implement block-level optimizations
- [ ] Support blocks with more than 255 children
- [ ] Use Reference atom
- [ ] Implement document viewer in CLI
- [ ] Optimize performance
- [ ] Put process in place for proposing new blocks
- [ ] Implement intermediary language for editing SemDocs
