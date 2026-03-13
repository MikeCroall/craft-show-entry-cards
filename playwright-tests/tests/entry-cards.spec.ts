import { test, expect } from "@playwright/test"

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

    // todo test interaction with the actual form and download
})
