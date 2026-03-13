import { defineConfig, devices } from "@playwright/test"

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
    testDir: "./tests",
    fullyParallel: true,

    forbidOnly: !!process.env.CI,
    retries: 6, // firefox, webkit, and mobile safari are incredibly flaky on privacy banner test case
    workers: process.env.CI ? 1 : undefined,

    reporter: "html",

    use: {
        // Base URL to use in actions like `await page.goto('')`
        baseURL: "http://localhost:8080/craft-show-entry-cards/",

        // Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer
        trace: "on-first-retry",
        screenshot: "only-on-failure",
    },

    projects: [
        {
            name: "chromium",
            use: { ...devices["Desktop Chrome"] },
        },
        {
            name: "firefox",
            use: { ...devices["Desktop Firefox"] },
        },
        {
            name: "webkit",
            use: { ...devices["Desktop Safari"] },
        },
        {
            name: "Mobile Chrome",
            use: { ...devices["Pixel 5"] },
        },
        {
            name: "Mobile Safari",
            use: { ...devices["iPhone 12"] },
        },

        /* Test against branded browsers. */
        // {
        //   name: 'Microsoft Edge',
        //   use: { ...devices['Desktop Edge'], channel: 'msedge' },
        // },
        // {
        //   name: 'Google Chrome',
        //   use: { ...devices['Desktop Chrome'], channel: 'chrome' },
        // },
    ],

    // Run the local dev server before starting the tests
    webServer: {
        name: "trunk serve",
        cwd: "../",
        command: "trunk serve",
        url: "http://localhost:8080/craft-show-entry-cards/",
        reuseExistingServer: !process.env.CI,
        stdout: "pipe",
        stderr: "pipe",
        timeout: 300_000,
    },
})
