import { expect, test } from "@playwright/test";

const baseURL = process.env.SPARSH_COUNTER_URL ?? "http://127.0.0.1:4177";

test("counter starter paints the Material-style shell and increments", async ({
  page,
}) => {
  await page.setViewportSize({ width: 430, height: 760 });
  await page.goto(baseURL);

  await expect(page).toHaveTitle(/Sparsha Counter/);
  await expect(
    page
      .locator(".sparsha-dom-root > div:not(.sparsha-semantic-root)")
      .filter({ hasText: "Sparsha Demo Home Page" })
      .first(),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", {
      name: "Sparsha Demo Home Page",
      level: 1,
    }),
  ).toBeVisible();
  await expect(
    page.getByText("You have pushed the button this many times:", {
      exact: true,
    }),
  ).toBeVisible();
  await expect(page.getByText("0", { exact: true }).first()).toBeVisible();

  const incrementButton = page.getByRole("button", { name: "Increment counter" });
  await expect(page.locator(".sparsha-semantic-root button")).toHaveAttribute(
    "type",
    "button",
  );
  await incrementButton.click({ force: true });
  await expect(page.getByText("1", { exact: true }).first()).toBeVisible();
});
