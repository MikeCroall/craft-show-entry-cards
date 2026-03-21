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
    expect(actual, `Found ${actual} occurrences of "${text}"`).toBe(expected)
}
