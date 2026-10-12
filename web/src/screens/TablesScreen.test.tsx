import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { fakeApi, renderApp } from "../test/renderWithApp";

describe("TablesScreen", () => {
  it("shows the table of 2, then the chosen one, from the core", async () => {
    const api = fakeApi();
    renderApp(api, "/tables");
    const user = userEvent.setup();

    const two = await screen.findByRole("region", { name: "Table de 2" });
    expect(within(two).getAllByRole("listitem")).toHaveLength(11);

    await user.click(screen.getByRole("button", { name: "7" }));
    const seven = await screen.findByRole("region", { name: "Table de 7" });
    expect(within(seven).getAllByRole("listitem")[3]).toHaveTextContent("7×3=21");
    expect(api.getMultiplicationTable).toHaveBeenCalledWith(7);
  });
});
