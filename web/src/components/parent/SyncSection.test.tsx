import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ApiError } from "../../api/client";
import { resetAutomaticSync } from "../../app/automaticSync";
import { lockParentSpace } from "../../app/parentAccess";
import { computerSyncStatus, fakeApi, lea, renderApp, tabletSyncStatus } from "../../test/renderWithApp";

async function enterParentSpace() {
  const user = userEvent.setup();
  await user.type(await screen.findByRole("textbox", { name: "Résultat du calcul" }), "162{Enter}");
  return user;
}

const tablet = { id: "tab-1", name: "Tablette 1", pairedAt: "2026-10-15T18:00:00.000Z", lastSyncAt: "2026-10-15T18:30:00.000Z" };

describe("Synchronisation in the parents' space (SUB027603)", () => {
  beforeEach(() => {
    lockParentSpace();
    resetAutomaticSync();
  });

  it("on the computer: listens while shown, shows the pairing code, and stops when left", async () => {
    const api = fakeApi();
    renderApp(api, "/parent");
    const user = await enterParentSpace();

    const section = await screen.findByRole("region", { name: "Synchronisation avec la tablette" });
    await waitFor(() => expect(api.openSyncScreen).toHaveBeenCalled());
    expect(within(section).getByText(/L'ordinateur attend la tablette sur le Wi-Fi \(192\.168\.1\.10:47615\)/)).toBeInTheDocument();
    expect(within(section).getByText("Aucune tablette associée pour l'instant.")).toBeInTheDocument();

    await user.click(within(section).getByRole("button", { name: "Associer une tablette" }));
    expect(await within(section).findByRole("img", { name: "Code à scanner avec la tablette" })).toBeInTheDocument();
    expect(within(section).getByText("code ABCD-EFGH")).toBeInTheDocument();
    expect(within(section).getByText(/valable 5 minutes et ne sert qu'une fois/)).toBeInTheDocument();
    await user.click(within(section).getByRole("button", { name: "Fermer le code" }));
    expect(api.cancelSyncPairing).toHaveBeenCalled();

    await user.click(within(section).getByRole("checkbox", { name: /Recevoir les synchronisations/ }));
    expect(api.setSyncReceive).toHaveBeenCalledWith(true);

    await user.click(screen.getByRole("button", { name: "← Retour au menu enfant" }));
    await waitFor(() => expect(api.closeSyncScreen).toHaveBeenCalled());
  });

  it("on the computer: lists the tablets and removes one after confirmation", async () => {
    const api = fakeApi({
      getSyncStatus: vi.fn().mockResolvedValue(computerSyncStatus([tablet])),
      openSyncScreen: vi.fn().mockResolvedValue(computerSyncStatus([tablet])),
      renameSyncDevice: vi.fn().mockResolvedValue(computerSyncStatus([{ ...tablet, name: "iPad du salon" }])),
    });
    renderApp(api, "/parent");
    const user = await enterParentSpace();

    const section = await screen.findByRole("region", { name: "Synchronisation avec la tablette" });
    expect(await within(section).findByText("Tablette 1")).toBeInTheDocument();
    expect(within(section).getByText(/Dernière synchronisation : 15 oct\./)).toBeInTheDocument();

    await user.click(within(section).getByRole("button", { name: "Renommer" }));
    const name = within(section).getByRole("textbox", { name: "Nouveau nom pour Tablette 1" });
    await user.clear(name);
    await user.type(name, "iPad du salon");
    await user.click(within(section).getByRole("button", { name: "Enregistrer" }));
    expect(api.renameSyncDevice).toHaveBeenCalledWith("tab-1", "iPad du salon");

    await user.click(await within(section).findByRole("button", { name: "Retirer iPad du salon" }));
    const dialog = within(section).getByRole("alertdialog");
    expect(within(dialog).getByText("Retirer iPad du salon ?")).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Retirer" }));
    expect(api.removeSyncDevice).toHaveBeenCalledWith("tab-1");
    expect(await within(section).findByText("Aucune tablette associée pour l'instant.")).toBeInTheDocument();
  });

  it("on the iPad: pairs with the short code when there is no camera, then synchronises", async () => {
    const api = fakeApi({
      getSyncStatus: vi.fn().mockResolvedValue(tabletSyncStatus(false)),
      openSyncScreen: vi.fn().mockResolvedValue(tabletSyncStatus(false)),
      scanPairingCode: vi.fn().mockRejectedValue(new ApiError(403, "Camera refused")),
    });
    renderApp(api, "/parent");
    const user = await enterParentSpace();

    const section = await screen.findByRole("region", { name: "Synchronisation avec l'ordinateur" });
    expect(await within(section).findByText("Cette tablette n'est associée à aucun ordinateur.")).toBeInTheDocument();
    await user.click(within(section).getByRole("button", { name: "Associer à l'ordinateur" }));
    expect(await within(section).findByText(/La caméra n'est pas autorisée/)).toBeInTheDocument();

    await user.type(within(section).getByRole("textbox", { name: "Adresse affichée par l'ordinateur" }), "192.168.1.10:47615");
    await user.type(within(section).getByRole("textbox", { name: "Code affiché par l'ordinateur" }), "abcd-efgh");
    await user.click(within(section).getByRole("button", { name: "Associer" }));

    expect(api.pairWithComputer).toHaveBeenCalledWith({ address: "192.168.1.10:47615", code: "abcd-efgh" });
    await waitFor(() => expect(api.syncNow).toHaveBeenCalledWith(false));
    expect(await within(section).findByText("Synchronisation terminée.")).toBeInTheDocument();
  });

  it("on the iPad: the QR code read by the camera pairs at once", async () => {
    const link = "educator://pair?v=1&h=192.168.1.10&p=47615&f=AB&s=CD";
    const api = fakeApi({
      getSyncStatus: vi.fn().mockResolvedValue(tabletSyncStatus(false)),
      openSyncScreen: vi.fn().mockResolvedValue(tabletSyncStatus(false)),
      scanPairingCode: vi.fn().mockResolvedValue(link),
    });
    renderApp(api, "/parent");
    const user = await enterParentSpace();

    const section = await screen.findByRole("region", { name: "Synchronisation avec l'ordinateur" });
    await user.click(await within(section).findByRole("button", { name: "Associer à l'ordinateur" }));

    await waitFor(() => expect(api.pairWithComputer).toHaveBeenCalledWith({ link }));
  });

  it("on the iPad: asks whether two profiles are the same child, then goes on", async () => {
    const question = {
      localProfileId: "p-lea-tablet",
      localName: "Léa",
      remoteProfileId: "p-lea",
      remoteName: "Léa",
      remoteCharacterId: "lion",
    };
    const api = fakeApi({
      getSyncStatus: vi.fn().mockResolvedValue(tabletSyncStatus(true, [question])),
      openSyncScreen: vi.fn().mockResolvedValue(tabletSyncStatus(true, [question])),
    });
    renderApp(api, "/parent");
    const user = await enterParentSpace();

    const group = await screen.findByRole("group", { name: "« Léa » existe sur la tablette et sur l'ordinateur. Est-ce le même enfant ?" });
    await user.click(within(group).getByRole("button", { name: "Oui, c'est le même enfant" }));

    expect(api.answerSyncQuestion).toHaveBeenCalledWith("p-lea-tablet", true);
    await waitFor(() => expect(api.syncNow).toHaveBeenCalledWith(false));
  });

  it("on the iPad: says simply why a synchronisation did not happen, never blaming anyone", async () => {
    const api = fakeApi({
      getSyncStatus: vi.fn().mockResolvedValue(tabletSyncStatus(true)),
      openSyncScreen: vi.fn().mockResolvedValue(tabletSyncStatus(true)),
      syncNow: vi.fn().mockResolvedValue({ outcome: "Unreachable", sent: 0, received: 0, status: tabletSyncStatus(true) }),
    });
    renderApp(api, "/parent");
    const user = await enterParentSpace();

    const section = await screen.findByRole("region", { name: "Synchronisation avec l'ordinateur" });
    expect(await within(section).findByText(/Associée à : Ordinateur/)).toBeInTheDocument();
    await user.click(within(section).getByRole("button", { name: "Synchroniser maintenant" }));

    expect(await within(section).findByText(/L'ordinateur est introuvable/)).toBeInTheDocument();
  });

  it("on the iPad: dissociates after confirmation", async () => {
    const api = fakeApi({
      getSyncStatus: vi.fn().mockResolvedValue(tabletSyncStatus(true)),
      openSyncScreen: vi.fn().mockResolvedValue(tabletSyncStatus(true)),
      removeSyncDevice: vi.fn().mockResolvedValue(tabletSyncStatus(false)),
    });
    renderApp(api, "/parent");
    const user = await enterParentSpace();

    const section = await screen.findByRole("region", { name: "Synchronisation avec l'ordinateur" });
    await user.click(await within(section).findByRole("button", { name: "Dissocier" }));
    await user.click(within(within(section).getByRole("alertdialog")).getByRole("button", { name: "Dissocier" }));

    expect(api.removeSyncDevice).toHaveBeenCalledWith("pc-1");
    expect(await within(section).findByText("Cette tablette n'est associée à aucun ordinateur.")).toBeInTheDocument();
  });
});

describe("Automatic synchronisation at the profile choice (SUB027602)", () => {
  beforeEach(() => resetAutomaticSync());

  it("on a paired iPad, synchronises once, silently", async () => {
    const api = fakeApi({ getSyncStatus: vi.fn().mockResolvedValue(tabletSyncStatus(true)) });
    renderApp(api, "/", null);

    expect(await screen.findByRole("button", { name: /Léa/ })).toBeInTheDocument();
    await waitFor(() => expect(api.syncNow).toHaveBeenCalledWith(true));
    expect(api.syncNow).toHaveBeenCalledTimes(1);
    expect(screen.queryByText(/Synchronisation/)).not.toBeInTheDocument();
  });

  it("does nothing on the computer, nor on an iPad without computer, nor when it fails", async () => {
    const computer = fakeApi();
    renderApp(computer, "/", null);
    await waitFor(() => expect(computer.getSyncStatus).toHaveBeenCalled());
    expect(computer.syncNow).not.toHaveBeenCalled();

    resetAutomaticSync();
    const failing = fakeApi({ getSyncStatus: vi.fn().mockRejectedValue(new ApiError(404, "Unknown command")) });
    renderApp(failing, "/", null);
    await waitFor(() => expect(failing.getSyncStatus).toHaveBeenCalled());
    expect(failing.syncNow).not.toHaveBeenCalled();
    expect(lea.name).toBe("Léa");
  });
});
