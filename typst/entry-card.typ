#set text(font: "Liberation Sans")

#let header = [
  #let year = datetime.today().year()
  = Bourne End Junior Craft Show #year
]

#let dottedLineFillWidth = [
  #set line(length: 100%)
  #line(stroke: (paint: black, thickness: 1pt, dash: "loosely-dotted"))
]

#let gridRequestedData(title) = [
  #grid(
    columns: (auto, 1fr),
    column-gutter: 0.9em,
    align: horizon,
    text(weight: "bold")[
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

#judgesSide
