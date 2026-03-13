import { test, expect, Page, Locator } from "@playwright/test"
import {
    assertHasFieldValueTwice,
    assertHasTextNTimes,
    loadPdf,
} from "../util/pdf-util"

const ID_PDF_EMBED = "#embed-pdf"
const SRC = "src"

const CONTACT_DETAILS = "Contact Details"
const CONTACT_DETAILS_VALUE = "john.smith@example.com"

const ENTRANTS_NAME = "Entrant's Name"
const ENTRANTS_NAME_VALUE = "Timmy Smith"

const ENTRANTS_AGE = "Entrant's Age"
const ENTRANTS_AGE_VALUE = "8"

test.describe("Entry Cards", () => {
    test.beforeEach(async ({ page }) => {
        await page.goto("") // uses config'd baseURL
    })

    test("has title", async ({ page }) => {
        await expect(page).toHaveTitle(/Entry Cards/)
    })

    test("has dismissable privacy banner that returns on reload", async ({
        page,
    }) => {
        const banner = page.locator("#privacy-banner")
        await expect(banner).toBeVisible()
        await expect(banner).toBeInViewport()

        const dismissButton = banner.getByRole("button", { name: "Dismiss" })
        await dismissButton.click()
        await expect(banner).not.toBeInViewport()
        await expect(banner).not.toBeVisible()

        await page.reload()
        await expect(banner).toBeVisible()
        await expect(banner).toBeInViewport()
    })

    test(`typing ${CONTACT_DETAILS} causes pdf render`, async ({ page }) => {
        await pdfRenderAfterInput(page, CONTACT_DETAILS, CONTACT_DETAILS_VALUE)
    })

    test(`typing ${ENTRANTS_NAME} causes pdf render`, async ({ page }) => {
        await pdfRenderAfterInput(page, ENTRANTS_NAME, ENTRANTS_NAME_VALUE)
    })

    test(`typing ${ENTRANTS_AGE} causes pdf render`, async ({ page }) => {
        await pdfRenderAfterInput(page, ENTRANTS_AGE, ENTRANTS_AGE_VALUE)
    })

    test(`typing ${CONTACT_DETAILS} & ${ENTRANTS_NAME} & ${ENTRANTS_AGE} causes pdf text to contain them`, async ({
        page,
    }) => {
        const pdfEmbed = page.locator(ID_PDF_EMBED)
        let pdfSrc = await pdfEmbed.getAttribute(SRC)

        await page.getByLabel(CONTACT_DETAILS).fill(CONTACT_DETAILS_VALUE)
        await srcToChange(pdfEmbed, pdfSrc)
        pdfSrc = await pdfEmbed.getAttribute(SRC)

        await page.getByLabel(ENTRANTS_NAME).fill(ENTRANTS_NAME_VALUE)
        await srcToChange(pdfEmbed, pdfSrc)
        pdfSrc = await pdfEmbed.getAttribute(SRC)

        await page.getByLabel(ENTRANTS_AGE).fill(ENTRANTS_AGE_VALUE)
        await srcToChange(pdfEmbed, pdfSrc)
        pdfSrc = await pdfEmbed.getAttribute(SRC)

        const downloadPromise = page.waitForEvent("download")
        await page.getByText("Save Pre-filled Entry Card PDF").click()
        const download = await downloadPromise

        expect(download.suggestedFilename()).toBe("prefilled-entry-card.pdf")
        const path = await download.path()
        const pdf = await loadPdf(path)
        expect(pdf.pageCount).toBe(1)

        // quick sense check of some contained text
        assertHasTextNTimes(pdf, "Section", 2)
        assertHasTextNTimes(pdf, "This side up for judging!", 2)
        assertHasTextNTimes(pdf, "Cut here", 2)
        assertHasTextNTimes(pdf, "Fold here", 2)

        // check specifically filled fields
        assertHasTextNTimes(pdf, CONTACT_DETAILS, 2)
        assertHasFieldValueTwice(pdf, CONTACT_DETAILS, CONTACT_DETAILS_VALUE)

        assertHasTextNTimes(pdf, ENTRANTS_NAME, 2)
        assertHasFieldValueTwice(pdf, ENTRANTS_NAME, ENTRANTS_NAME_VALUE)

        assertHasTextNTimes(pdf, ENTRANTS_AGE, 2)
        assertHasFieldValueTwice(pdf, ENTRANTS_AGE, ENTRANTS_AGE_VALUE)
    })
})

async function pdfRenderAfterInput(
    page: Page,
    inputLabel: string,
    fillValue: string,
) {
    const pdfEmbed = page.locator(ID_PDF_EMBED)
    const pdfBefore = await pdfEmbed.getAttribute(SRC)

    await page.getByLabel(inputLabel).fill(fillValue)

    await srcToChange(pdfEmbed, pdfBefore)
}

async function srcToChange(locator: Locator, prevSrc: string | null) {
    if (prevSrc) await expect(locator).not.toHaveAttribute(SRC, prevSrc)
    else await expect(locator).toHaveAttribute(SRC)
}
