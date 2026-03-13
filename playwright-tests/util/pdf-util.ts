import { expect } from "@playwright/test"
import { getDocument } from "pdfjs-dist/legacy/build/pdf.mjs"
import { TextItem, TextMarkedContent } from "pdfjs-dist/types/src/display/api"

type ExtractedPdf = {
    pageCount: number
    textContents: string[]
}

export async function loadPdf(pdfPath: string): Promise<ExtractedPdf> {
    const doc = await getDocument(pdfPath).promise
    const pageCount = doc.numPages
    const textContents: string[] = []

    const loadPage = async function (pageNum: number) {
        const page = await doc.getPage(pageNum)
        const content = await page.getTextContent()
        textContents.push(
            ...content.items
                .filter(isTextItem)
                .map(item => item.str.trim())
                .filter(str => str.length > 0),
        )
        page.cleanup()
    }

    let promiseChain
    promiseChain = doc.getMetadata()

    for (let i = 1; i <= pageCount; i++) {
        promiseChain = promiseChain.then(loadPage.bind(null, i))
    }
    await promiseChain

    return {
        pageCount,
        textContents,
    }
}

function isTextItem(item: TextItem | TextMarkedContent): item is TextItem {
    return (<TextItem>item).str !== undefined
}

function countOccurrences(pdf: ExtractedPdf, text: string): number {
    return pdf.textContents.filter(s => s === text).length
}

export function assertHasTextNTimes(
    pdf: ExtractedPdf,
    text: string,
    expected: number,
) {
    const actual = countOccurrences(pdf, text)
    expect(actual).toBe(expected)
}

/**
 * This value is specialised for the entry cards, not general purpose PDFs.
 * The entry cards will contain each field twice as we fit 2 to a sheet.
 * The rendering of the PDF means that in the parsed string[], each field
 * name is immediately followed by the value entered for it. This function
 * asserts that the value immediately following `fieldName` in the array equals
 * `expectedValue` twice.
 */
export function assertHasFieldValueTwice(
    { textContents }: ExtractedPdf,
    fieldName: string,
    expectedValue: string,
) {
    let found = 0
    for (let i = 0; i < textContents.length - 1; i++) {
        const element = textContents[i]
        if (element != fieldName) continue
        i++
        const value = textContents[i]
        if (value != expectedValue) continue
        found++
    }
    expect(found).toBe(2)
}
