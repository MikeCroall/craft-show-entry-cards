#set text(font: "Arial")

#let header = [
  #let year = datetime.today().year()
  = Bourne End Junior Craft Show #year
]

#let judges_side = [
  #header \
  *Class* ... \
  ... \
  *Entrant's Age* ... \

// todo replace ... with actual lines https://typst.app/docs/reference/visualize/stroke/

  #align(center, text(weight: "bold")[
    #underline[This side up for judging!]
  ])
]

#judges_side
