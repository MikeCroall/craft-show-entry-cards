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

#let contactSide = [
  #header \
  #gridRequestedData("Description or Title of Entry") \
  #dottedLineFillWidth \
  #dottedLineFillWidth \
  #gridRequestedData("Entrant's Name") \
  #gridRequestedData("Contact Details") \

  #align(center, [
    By entering, you agree to photos of your entry being used on our website
    and in local publications, and to storage of your contact details so you may be contacted
    if your entry wins a prize that you are not there to collect.
  ])
]

#let pageQuadrants = [
  #grid(
    columns: (1fr, 1fr),
    rows: (1fr, 1fr),
    gutter: 10pt,
    rotate(90deg, reflow: true)[#judgesSide],
    rotate(-90deg, reflow: true)[#contactSide],
    rotate(90deg, reflow: true)[#contactSide],
    rotate(-90deg, reflow: true)[#judgesSide],
  )
]

//#judgesSide
//#contactSide
#pageQuadrants
