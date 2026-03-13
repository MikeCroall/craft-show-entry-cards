import { test, expect, Page } from "@playwright/test"

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

        page.reload()
        await expect(banner).toBeVisible()
        await expect(banner).toBeInViewport()
    })

    test("typing contact details causes pdf render", async ({ page }) => {
        await typingCausesPdfRender(page, "Contact Details", "John Smith")
    })

    test("typing entrant name causes pdf render", async ({ page }) => {
        await typingCausesPdfRender(page, "Entrant's Name", "Timmy Smith")
    })

    test("typing entrant age causes pdf render", async ({ page }) => {
        await typingCausesPdfRender(page, "Entrant's Age", "8")
    })

    test.skip("typing contact details and entrant name and age causes pdf text to contain them", async ({
        page,
    }) => {
        // todo test interaction with the actual form and download
    })
})

async function typingCausesPdfRender(
    page: Page,
    inputLabel: string,
    fillValue: string,
) {
    const pdfEmbed = page.locator("#embed-pdf")
    const pdfBefore = pdfEmbed.getAttribute("src")

    await page.getByLabel(inputLabel).fill(fillValue)

    await expect.poll(() => pdfEmbed.getAttribute("src")).not.toBe(pdfBefore)
}
