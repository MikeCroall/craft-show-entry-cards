#import "@preview/curvly:0.1.0"

#let oldDiameterMM = 65mm
#let newDiameterMM = 54mm
#let rosetteMiddleDiameterMM = newDiameterMM

#let paddingMultiplier = 1.1
#let paddedSize = 1.1 * rosetteMiddleDiameterMM

#let textCircleMultiplier = 0.97

#let gradientInputs = (
  orange: rgb("#ff6f02"),
  orangeOffset: 18%,
  yellow: rgb("#fffe00"),
  angle: 40deg,
)

#let boundaryInputs = (
  color: rgb("#999"),
  width: rosetteMiddleDiameterMM,
)

#let topTextInputs = (
  color: black,
  size: 0.16 * rosetteMiddleDiameterMM,
  weight: 900,
  text: "Bourne End",
  angle: 130deg,
  font: "libertinus serif",
  circleWidth: textCircleMultiplier * rosetteMiddleDiameterMM,
)

#let bottomTextInputs = (
  color: black,
  size: 0.08 * rosetteMiddleDiameterMM,
  weight: 700,
  text: "Junior Craft Show",
  angle: 100deg,
  font: "libertinus serif",
  circleWidth: textCircleMultiplier * rosetteMiddleDiameterMM,
)

#let middleTextInputs = (
  color: red,
  sizeBig: 0.5 * rosetteMiddleDiameterMM,
  textBig: "1",
  sizeSmall: 0.2 * rosetteMiddleDiameterMM,
  textSmall: "ST",
  font: "libertinus serif",
)

#set document(title: [Rosettes])

#set page(
  paper: "a4",
  margin: (
    rest: 0em,
  ),
)

#let squareBacking = place(center + horizon, square(
  width: paddedSize,
  fill: gradient.linear(
    (gradientInputs.orange, 0%),
    (gradientInputs.orange, gradientInputs.orangeOffset),
    (gradientInputs.yellow, 50%),
    (gradientInputs.orange, 100% - gradientInputs.orangeOffset),
    (gradientInputs.orange, 100%),
    angle: gradientInputs.angle,
  ),
))

#let circleBoundary = place(
  center + horizon,
  circle(stroke: boundaryInputs.color, width: boundaryInputs.width),
)

#let topText = [
  #set text(topTextInputs.color, topTextInputs.size, font: topTextInputs.font, weight: topTextInputs.weight)
  #place(
    center + horizon,
    curvly.text-on-circle(
      topTextInputs.text,
      "",
      topTextInputs.circleWidth,
      topTextInputs.angle,
      0deg,
    ),
  )
]

#let bottomText = [
  #set text(bottomTextInputs.color, bottomTextInputs.size, font: bottomTextInputs.font, weight: bottomTextInputs.weight)
  #place(
    center + horizon,
    curvly.text-on-circle(
      "",
      bottomTextInputs.text,
      bottomTextInputs.circleWidth,
      0deg,
      bottomTextInputs.angle,
    ),
  )
]

#let middleText = [
  #set text(middleTextInputs.color, font: middleTextInputs.font)
  #place(
    center + horizon,
    stack(
      dir: ltr,
      text(middleTextInputs.textBig, size: middleTextInputs.sizeBig),
      align(
        bottom,
        text(middleTextInputs.textSmall, size: middleTextInputs.sizeSmall),
      ),
    ),
  )
]

#let rosette = block(
  width: paddedSize,
  height: paddedSize,
  [
    #squareBacking
    #circleBoundary
    #topText
    #bottomText
    #middleText
  ],
)

#place(
  top + left,
  stack(
    dir: ttb,
    spacing: 0.2em,
    [#text("Rosette middle size:") #rosetteMiddleDiameterMM.mm() #text("mm")],
    line(length: rosetteMiddleDiameterMM),
  ),
)

#align(
  center + horizon,
  layout(size => {
    grid(
      columns: int(size.width / paddedSize),
      rows: auto,
      gutter: if rosetteMiddleDiameterMM == oldDiameterMM { auto } else {
        1em
      },
      align: center,
      rosette, rosette, rosette,
      rosette, rosette, rosette,
      rosette, rosette, rosette,
      rosette, rosette, rosette,
    )
  }),
)
