## SemDoc – Semantic Document

SemDoc is a file format that empowers writers to solely focus on the content and allows readers to customize the appearance and adapt it to any surface.

### What's wrong with the world

You're wondering why we need yet another file format?
Suppose you want to send someone an essay you wrote. It contains typical stuff: Some text, images, etc.
What format do you use?

You could send them a file in the proprietary Microsoft Word format, but that would require them to either pay Microsoft or install another editing program attempting to understand the file.
Of course, these programs are usually more optimized for writing than reading text, so they'll also have a cluttered screen with lots of toolbars.

Maybe a Markdown file? Although by default, that will probably get opened in an editor with monospace font, not in a rich preview. Not seeing images will confuse non-technical users.

You could send them a Latex file, but it'll take some time until they get it to build.

It turns out, the go-to choice for most people is PDF – by a large margin.
Assignments from university? PDF.
Scientific papers? PDF.
Communication with authorities of the country? PDF.
Heck, the mails of my landlord's association simply say "The content of this mail is in the attached PDF".

Over time, I learned to despise PDFs. I don't even believe that the format is inherently bad, it's just used for things that it wasn't designed for.
It's completely ill-fitted for today's world:

* **Everything is organized into pages.** This might have made sense back when you printed everything, but nowadays, we just view PDFs on our screens. It's not like scrolling is a particularly new invention.
* **It's completely static.**
  This is probably the biggest downside.
  I mean, instead of a PDF I could just as easily give you a vector image that you can zoom around in (well, you can't select text, but other than that it's just as adaptive).  
  You want to view a PDF on a big landscape screen? Say hello to inefficient usage of space.  
  And don't get me started on viewing my homework assignments on the train. I refuse to believe that having to put my phone into landscape mode just so I can read a single line without constantly scrolling back and forth is the best user experience we can come up with in the 21st century.
* **The advanced features are broken.**
  Yes, PDFs support more features than vector images but have you ever encountered a PDF with input fields where the tab navigation wasn't completely arbitrary, and copying text with accents worked?  
  A small hint: If almost nobody can produce usable documents with your format, that's probably a usability issue with the format itself.
* **Dark mode?** Anyone?

Maybe I'm coming off a little harsh. It's not like PDF has no use cases – if I want to print something, sending a PDF to the printer usually works great.
And if you're a company that wants to impress others with a design brochure that uses custom colors, fonts, and layouts and your target audience are people viewing the document on a vertical monitor in bright daylight, it's your right to produce a perfect experience for them.

### Relevance

Some might think, "documents are going to be cloud-first anyway. Consumers don't need file formats anymore; they'll collaborate online."
That's an interesting view. But I'd argue there will always be a use-case for immutable atomic instances of documents to be sent.

Whether it's because immutability makes them legally binding or because latency forbids you from collaborating with people living on Mars.
**The concept of transferring a file is very easy to understand for us humans because files are like things.**

### Goals

SemDoc follows two principles:

* **It's a compile target.**
  It doesn't aim to be readable to humans or be edited by hand; instead, it's an efficient binary format.
  Like PDFs, you compile other formats to it.
  Edit in Markdown, Google Docs, Notion, Latex, or whatever else and compile to SemDoc.
* **Purely semantic.**
  It contains no syntax information, only semantic information.
  This allows SemDoc viewers to choose the optimal way of presenting the information and gives them a lot of freedom.

**Together, these move all the control over the document's appearance to the reader of the document.**

I can almost hear people asking:
But what about just a tiny bit of syntax? I'm only asking for …

* **customizing the color scheme?**
  What if I told you some operating systems (for example, Windows) let the user choose a custom accent color?
  What about light and dark mode?
  What about blue mode? Red mode?
  What about AR glasses where the world is the background?

* **overriding the default fonts?**
  Explain that to people with dyslexia, who use special fonts that give each letter a recognizable characteristic.
  Debate with people who practised speed reading with one particular font.
  And I can almost see the hacked-together "music fonts" and "math fonts" popping up for allowing you distribute other content.
  Why bloat the document with font definitions?

* **make text bold?**
  You will be able to mark content as *important*, but "bold" is such an arbitrary property.
  What about smart speakers reading the text out loud? Should they speak boldly?
  Why not give developers of AR glasses the freedom to mark important text by lifting it a little bit to the front?
  Why artificially limit yourself to a concept that only makes sense on 2D screens?

Also note that these little customization options all add up.
Over time, peer-pressure might build up that forces every writer to also think of a fancy color scheme (of course, for light and dark mode), provide a font, etc.  
That's not what writers should have to concern themselves with.
If you do want more control, you're welcome to build a website.

Not defining the appearance also makes the format future-proof – it adapts to current devices just as well as to AR glasses.
And a long-lived format is a win for everyone using it.
