#import sys: inputs

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

#let gridRequestedData(title, value: none) = [
  #grid(
    columns: (auto, 1fr),
    column-gutter: 0.7em,
    align: horizon,
    text(size: 1.5em)[
      #title
    ],
    align(bottom, [
      #if value != none { text(size: 1.5em)[ // TODO handwriting font?
          #value // TODO support showing dotted line even when value is present
        ]} else { dottedLineFillWidth }
    ])
  )
]

#let entrants_age = if ("entrants_age" in inputs) { inputs.entrants_age } else { none }
#let judgesSideStack = align(left, stack(
  dir: ttb,
  spacing: 3em,

  header,

  gridRequestedData("Class"),
  dottedLineFillWidth,
  gridRequestedData("Entrant's Age", value: entrants_age),

  align(center, text(weight: "bold")[
    #underline(evade:false, offset: 1pt, [
      This side up for judging!
    ])
  ])
))


#let entrants_name = if ("entrants_name" in inputs) { inputs.entrants_name } else { none }
#let contact_details = if ("contact_details" in inputs) { inputs.contact_details } else { none }
#let contactSide = align(left, stack(
  dir: ttb,
  spacing: 3em,

  header,

  gridRequestedData("Description or Title of Entry"),
  dottedLineFillWidth,
  dottedLineFillWidth,
  gridRequestedData("Entrant's Name", value: entrants_name),
  gridRequestedData("Contact Details", value: contact_details),

  align(center, text(size: 10pt)[
    By entering, you agree to photos of your entry being used on our website
    and in local publications, and to storage of your contact details so you may be contacted
    if your entry wins a prize that you are not there to collect.
  ])
))

#let pageQuadrants = [
  #grid(
    columns: (1fr, 1fr),
    rows: (1fr, 1fr),
    inset: 10pt,

    align(right, rotate(90deg, reflow: true)[#judgesSideStack]), // top left
    grid.vline(stroke:(paint: black, thickness: 1pt, dash: "loosely-dash-dotted")),
    align(left, rotate(-90deg, reflow: true)[#contactSide]), // top right
    grid.hline(stroke:(paint: black, thickness: 1pt)),
    align(right, rotate(90deg, reflow: true)[#contactSide]), // bottom left
    align(left, rotate(-90deg, reflow: true)[#judgesSideStack]), // bottom right
  )
]

#pageQuadrants

#place(
  horizon + left,
  dx: 50% - 4em,
  box(fill: white, image("icons/scissors.svg", height: 2em))
)

#place(
  top + center,
  dy: 50% - 4em,
  box(fill: white, image("icons/fold.svg", height: 1.75em))
)

/*
todo check the following fields on `inputs` and render if present
contact_details
entrants_name
entrants_age
*/
