#set page(margin: (
  rest: 0.5em,
))

#set text(font: "Liberation Sans")

#let header = [
  #let year = datetime.today().year()
  #text(size: 1.8em)[Bourne End Junior Craft Show #year] \
]

#let dottedLineFillWidth = [
  #set line(length: 100%)
  #line(stroke: (paint: black, thickness: 1pt, dash: "loosely-dotted"))
]

#let gridRequestedData(title) = [
  #grid(
    columns: (auto, 1fr),
    column-gutter: 0.7em,
    align: horizon,
    text(size: 1.5em)[
      #title
    ],
    align(bottom, [
      #dottedLineFillWidth
    ])
  )
]

#let judgesSide = [
  #header \
  #gridRequestedData("Class") \
  #dottedLineFillWidth \
  #gridRequestedData("Entrant's Age") \

  #align(center, text(weight: "bold")[
    #underline[This side up for judging!]
  ])
]

#let contactSide = [
  #header \
  #gridRequestedData("Description or Title of Entry") \
  #dottedLineFillWidth \
  #dottedLineFillWidth \
  #gridRequestedData("Entrant's Name") \
  #gridRequestedData("Contact Details") \

  #align(center, text(size: 10pt)[
    By entering, you agree to photos of your entry being used on our website
    and in local publications, and to storage of your contact details so you may be contacted
    if your entry wins a prize that you are not there to collect.
  ])
]

#let pageQuadrants = [
  #grid(
    columns: (1fr, 1fr),
    rows: (1fr, 1fr),
    row-gutter: 10pt,
    column-gutter: 10pt,
    align(right, rotate(90deg, reflow: true)[#align(left, judgesSide)]), // top left
    align(left, rotate(270deg, reflow: true)[#align(left, contactSide)]), // top right
    align(right, rotate(90deg, reflow: true)[#align(left, contactSide)]), // bottom left
    align(left, rotate(270deg, reflow: true)[#align(left, judgesSide)]), // bottom right
  )
]

#pageQuadrants
